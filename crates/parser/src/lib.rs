#![feature(box_syntax, box_patterns)]
extern crate python_ast as ast;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate error_chain;

use std::process::Command;
use std::str;
use std::fmt;
use nom::IResult;

pub mod error;
pub use error::Error;
pub use error::ErrorKind;
pub use error::Result;
pub use error::ResultExt;

pub type BoxNode<'a> = Box<Node<'a>>;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    // Module(doc, node)
    Module(&'a str, BoxNode<'a>),
    // Stmt(nodes)
    Stmt(Vec<Node<'a>>),
    // Const(value)
    Const(&'a str),
    // Discard(expr)
    Discard(BoxNode<'a>),
    // Name(name)
    Name(&'a str),
    // Printnl(nodes, dest)
    Printnl(Vec<Node<'a>>, &'a str),
    // Assign(nodes, expr)
    Assign(Vec<Node<'a>>, BoxNode<'a>),
    // AssName(name, flags)
    AssignName(&'a str, &'a str),
    // Add((left, right))
    Add(BoxNode<'a>, BoxNode<'a>),
    // UnarySub(expr)
    UnarySub(BoxNode<'a>),
    // CallFunc(node, args)
    CallFunc(BoxNode<'a>, Vec<Node<'a>>),
    // Compare(expr, ops)
    // 1 == 2 == 3
    // Compare(Const(1), [('==', Const(2)), ('==', Const(3))])
    Compare(BoxNode<'a>, Vec<(&'a str, Node<'a>)>),
    // Or(nodes)
    Or(Vec<Node<'a>>),
    // And(nodes)
    And(Vec<Node<'a>>),
    // Not(expr)
    Not(BoxNode<'a>),
    // List(nodes)
    List(Vec<Node<'a>>),
    // Dict(items)
    Dict(Vec<(Node<'a>, Node<'a>)>),
    // Subscript(expr, flags, subs)
    Subscript(BoxNode<'a>, &'a str, Vec<Node<'a>>),
    // IfExp(test, then, else)
    IfExp(BoxNode<'a>, BoxNode<'a>, BoxNode<'a>),
}

macro_rules! lowering_err {
    ($node:expr, $to:ty) => ({
        ErrorKind::Lowering(format!("{:?}", $node), format!("{}", stringify!($to)))
    })
}

impl<'a> Node<'a> {
    pub fn module_into_ast(self) -> Result<ast::Program> {
        use Node::*;
        let stmts: Vec<Node> = match self {
            Module(_, box Stmt(nodes)) => nodes,
            _ => panic!("Expected module with statements at top level!"),
        };
        let mut statements: Vec<ast::Statement> = vec![];
        for stmt in stmts {
            match stmt {
                Discard(box Const("None")) => continue,
                _ => {}
            }
            let statement = stmt.clone().lower_to_stmt().chain_err(|| {
                lowering_err!(stmt, ast::Statement)
            })?;
            statements.push(statement);
        }
        let program = ast::Program { module: ast::Module { statements } };
        return Ok(program);
    }

    pub fn lower_to_stmt(self) -> Result<ast::Statement> {
        use Node::*;
        let statement = match self {
            Module(_, _) | Stmt(_) => return Err(lowering_err!(self, ast::Statement).into()),
            Discard(box node) => node.lower_to_stmt()?,
            Printnl(nodes, dest) => {
                let e = nodes[0].clone().lower_to_expr()?;
                ast::Statement::Print(e)
            }
            Assign(nodes, box expr) => {
                let target = nodes[0].clone().lower_to_target()?;
                let expr = expr.lower_to_expr()?;
                ast::Statement::Assign(target, expr)
            }
            e @ Const(_) |
            e @ Name(_) |
            // TODO should we be lowering assign names to exprs?
            e @ AssignName(_, _) |
            e @ Add(_, _) |
            e @ UnarySub(_) |
            e @ CallFunc(_, _) |
            e @ Compare(_, _) |
            e @ Or(_) |
            e @ And(_) |
            e @ Not(_) |
            e @ List(_) |
            e @ Dict(_) |
            e @ Subscript(_, _, _) |
            e @ IfExp(_, _, _) => {
                let e = e.lower_to_expr()?;
                ast::Statement::Expression(e)
            }
        };

        return Ok(statement);
    }

    pub fn lower_to_expr(self) -> Result<ast::Expression> {
        use Node::*;
        let expression = match self {
            Const(val) => {
                match val {
                    "True" => ast::Expression::Boolean(true),
                    "False" => ast::Expression::Boolean(false),
                    //"None" => ast::Expression::
                    int => {
                        let i = int.parse().chain_err(|| {
                            format!("Unable to parse integer from {:?}", Const(val))
                        })?;
                        ast::Expression::DecimalI32(i)
                    }
                }
            }
            Add(box left, box right) => {
                let left = left.lower_to_expr()?;
                let right = right.lower_to_expr()?;
                ast::Expression::Add(box left, box right)
            }
            UnarySub(box Const(int)) if int != "True" && int != "False" => {
                let int = format!("-{}", int);
                Const(&int).lower_to_expr()?
            }
            UnarySub(box node) => ast::Expression::UnaryNeg(box node.lower_to_expr()?),
            CallFunc(box node, _args) => {
                match node {
                    Name("input") => ast::Expression::Input,
                    Name(name) => {
                        let err = ErrorKind::NonInputCallExpr(name.into());
                        bail!(err);
                    }
                    node => {
                        let err = ErrorKind::NonNameCallTarget(format!("{:?}", node));
                        bail!(err);
                    }
                }
            }
            Compare(box first, nodes) => {
                fn cmp(
                    operator: &str,
                    left: ast::Expression,
                    right: ast::Expression,
                ) -> ast::Expression {
                    match operator {
                        "==" => ast::Expression::LogicalEq(box left, box right),
                        "!=" => ast::Expression::LogicalNotEq(box left, box right),
                        operator => {
                            let err = ErrorKind::UnexpectedCompareOp(operator.to_string());
                            return ast::Expression::Input;
                        }
                    }
                }
                // reverse in order to preserve left to right ordering
                let mut nodes = nodes.into_iter();
                let first = first.lower_to_expr()?;
                // expect at least one
                let (op, node) = nodes.next().unwrap();
                // union of all equals with "and"
                let mut chained_and_expr = cmp(op, first.clone(), node.lower_to_expr()?);
                for (op, node) in nodes {
                    let e = cmp(op, first.clone(), node.lower_to_expr()?);
                    chained_and_expr = ast::Expression::LogicalAnd(box chained_and_expr, box e);
                }
                chained_and_expr
            }
            Or(nodes) => {
                let mut nodes = nodes.into_iter().rev();
                // expected at least 2
                let mut right = nodes.next().unwrap().lower_to_expr()?;
                let mut left = nodes.next().unwrap().lower_to_expr()?;
                let mut chained_or_expr = ast::Expression::LogicalOr(box left, box right);
                for node in nodes {
                    right = chained_or_expr;
                    left = node.lower_to_expr()?;
                    chained_or_expr = ast::Expression::LogicalOr(box left, box right);
                }
                chained_or_expr
            }
            And(nodes) => {
                let mut nodes = nodes.into_iter();
                let mut right = nodes.next().unwrap().lower_to_expr()?;
                let mut left = nodes.next().unwrap().lower_to_expr()?;
                let mut chained_and_expr = ast::Expression::LogicalAnd(box left, box right);
                for node in nodes {
                    right = chained_and_expr;
                    left = node.lower_to_expr()?;
                    chained_and_expr = ast::Expression::LogicalAnd(box left, box right);
                }
                chained_and_expr
            }
            Not(box node) => ast::Expression::LogicalNot(box node.lower_to_expr()?),
            List(nodes) => ast::Expression::List({
                let nodes: Result<Vec<_>> = nodes.into_iter().map(|n| n.lower_to_expr()).collect();
                nodes?
            }),
            Dict(tuples) => ast::Expression::Dict({
                let mut dict: Vec<(ast::Expression, ast::Expression)> = vec![];
                for (l, r) in tuples {
                    let left = l.lower_to_expr()?;
                    let right = r.lower_to_expr()?;
                    dict.push((left, right))
                }
                dict
            }),
            target @ Name(_) |
            target @ Subscript(_, _, _) |
            target @ AssignName(_, _) => ast::Expression::Target(target.lower_to_target()?),
            Module(_, _) => panic!("lowering module to expr"),
            Stmt(_) => panic!("lowering stmt to expr"),
            Discard(box node) => node.lower_to_expr()?,
            Printnl(_, _) => panic!("lowering println to expr"),
            Assign(_, _) => panic!("lowering assign to expr"),
            IfExp(test, then, els) => {
                ast::Expression::If(
                    box test.lower_to_expr()?,
                    box then.lower_to_expr()?,
                    box els.lower_to_expr()?,
                )
            }
        };

        return Ok(expression);
    }

    pub fn lower_to_target(self) -> Result<ast::Target> {
        use Node::*;
        let target = match self {
            Name(name) => ast::Target::Name(name.into()),
            AssignName(name, _) => ast::Target::Name(name.into()),
            Subscript(box target, _, subs) => {
                ast::Target::Subscript(
                    box target.lower_to_expr()?,
                    box subs[0].clone().lower_to_expr()?,
                )
            }
            node => {
                let err = ErrorKind::Lowering(format!("{:?}", node), "ast::Target".into());
                bail!(err);
            }
        };

        return Ok(target);
    }
}

named!(
    node<Node>,
    alt!(
        map!(module, |(doc, node)| Node::Module(doc, Box::new(node)))           |
        map!(stmt, |nodes| Node::Stmt(nodes))                                   |
        map!(constant, |val| Node::Const(val))                                  |
        map!(discard, |node| Node::Discard(Box::new(node)))                     |
        map!(name, |name| Node::Name(name))                                     |
        map!(printnl, |(nodes, dest)| Node::Printnl(nodes, dest))               |
        map!(assign, |(nodes, expr)| Node::Assign(nodes, Box::new(expr)))       |
        map!(assign_name, |(name, flags)| Node::AssignName(name, flags))        |
        map!(add, |(l, r)| Node::Add(Box::new(l), Box::new(r)))                 |
        map!(unary_sub, |node| Node::UnarySub(Box::new(node)))                  |
        map!(call_func, |(node, args)| Node::CallFunc(Box::new(node), args))    |
        map!(compare, |(node, ops)| Node::Compare(Box::new(node), ops))         |
        map!(or, |nodes| Node::Or(nodes))                                       |
        map!(and, |nodes| Node::And(nodes))                                     |
        map!(not, |node| Node::Not(Box::new(node)))                             |
        map!(list, |nodes| Node::List(nodes))                                   |
        map!(dict, |tuples| Node::Dict(tuples))                                 |
        map!(subscript, |(target, flags, subs)| Node::Subscript(Box::new(target), flags, subs)) |
        map!(if_exp, |(test, then, els)| Node::IfExp(Box::new(test), Box::new(then), Box::new(els)))
    )
);

named!(
    module<&[u8], (&str, Node)>,
    do_parse!(
        tag!("Module") >>
        doc: delimited!(tag!("("), is_not!(","), tag!(",")) >>
        node: ws!(node) >>
        tag!(")") >>
        ((to_str(doc), node))
    )
);

named!(
    stmt<Vec<Node>>,
    do_parse!(
        tag!("Stmt") >>
        nodes: delimited!(tag!("("), node_list, tag!(")")) >>
        (nodes)
    )
);

named!(
    discard<Node>,
    do_parse!(
        tag!("Discard") >>
        node: delimited!(tag!("("), node, tag!(")")) >>
        (node)
    )
);

named!(node_list<Vec<Node>>, delimited!(alt!(tag!("[") | tag!("(")), separated_list_complete!(tag!(","), ws!(node)), alt!(tag!("]") | tag!(")"))));

named!(
    node_tuple<(Node, Node)>,
    do_parse!(
        left: delimited!(tag!("("), node, tag!(",")) >>
        right: terminated!(ws!(node), tag!(")")) >>
        ((left, right))
    )
);

named!(
    tuple_list<Vec<(Node, Node)>>,
    do_parse!(
        tuples: delimited!(alt!(tag!("[") | tag!("(")), separated_list_complete!(tag!(","), ws!(node_tuple)), alt!(tag!("]") | tag!(")"))) >>
        (tuples)
    )
);

named!(
    constant<&str>,
    do_parse!(
        tag!("Const") >>
        v: delimited!(tag!("("), is_not!(")"), tag!(")")) >>
        (to_str(v))
    )
);

named!(
    name<&str>,
    do_parse!(
        tag!("Name") >>
        name: delimited!(tag!("("), string_literal, tag!(")")) >>
        (name)
    )
);

named!(
    printnl<(Vec<Node>, &str)>,
    do_parse!(
        tag!("Printnl") >>
        nodes: delimited!(tag!("("), node_list, tag!(",")) >>
        dest: terminated!(ws!(is_not!(")")), tag!(")")) >>
        (nodes, to_str(dest))
    )
);

named!(
    assign<(Vec<Node>, Node)>,
    do_parse!(
        tag!("Assign") >>
        nodes: delimited!(tag!("("), node_list, tag!(",")) >>
        expr: terminated!(ws!(node), tag!(")")) >>
        (nodes, expr)
    )
);

named!(
    assign_name<(&str, &str)>,
    do_parse!(
        tag!("AssName") >>
        name: delimited!(tag!("("), string_literal, tag!(",")) >>
        flags: terminated!(ws!(string_literal), tag!(")")) >>
        (name, flags)
    )
);

named!(
    add<(Node, Node)>,
    do_parse!(
        tag!("Add") >>
        left: delimited!(tag!("(("), node, tag!(",")) >>
        right: terminated!(ws!(node), tag!("))")) >>
        ((left, right))
    )
);

named!(
    unary_sub<Node>,
    do_parse!(
        tag!("UnarySub") >>
        node: delimited!(tag!("("), node, tag!(")")) >>
        (node)
    )
);

named!(
    call_func<(Node, Vec<Node>)>,
    do_parse!(
        tag!("CallFunc") >>
        n: delimited!(tag!("("), node, tag!(",")) >>
        args: terminated!(ws!(node_list), tag!(",")) >>
        terminated!(ws!(tag!("None")), tag!(",")) >>
        terminated!(ws!(tag!("None")), tag!(")")) >>
        ((n, args))
    )
);

named!(
    compare<(Node, Vec<(&str, Node)>)>,
    do_parse!(
        tag!("Compare") >>
        e: delimited!(tag!("("), node, tag!(",")) >>
        o: terminated!(ws!(delimited!(tag!("["), separated_list_complete!(tag!(","), ws!(compare_op)), tag!("]"))), tag!(")")) >>
        ((e, o))
    )
);

named!(
    compare_op<(&str, Node)>,
    do_parse!(
        op: delimited!(tag!("("), is_not!(","), tag!(",")) >>
        node: terminated!(ws!(node), tag!(")")) >>
        ((to_str(op), node))
    )
);

named!(
    or<Vec<Node>>,
    do_parse!(
        tag!("Or") >>
        nodes: delimited!(tag!("("), node_list, tag!(")")) >>
        (nodes)
    )
);

named!(
    and<Vec<Node>>,
    do_parse!(
        tag!("And") >>
        nodes: delimited!(tag!("("), node_list, tag!(")")) >>
        (nodes)
    )
);

named!(
    not<Node>,
    do_parse!(
        tag!("Not") >>
        node: delimited!(tag!("("), node, tag!(")")) >>
        (node)
    )
);

named!(
    list<Vec<Node>>,
    do_parse!(
        tag!("List") >>
        nodes: delimited!(tag!("("), node_list, tag!(")")) >>
        (nodes)
    )
);

named!(
    dict<Vec<(Node, Node)>>,
    do_parse!(
        tag!("Dict") >>
        tuples: delimited!(tag!("("), tuple_list, tag!(")")) >>
        (tuples)
    )
);

named!(
    subscript<(Node, &str, Vec<Node>)>,
    do_parse!(
        tag!("Subscript") >>
        target: delimited!(tag!("("), node, tag!(",")) >>
        flags: terminated!(ws!(is_not!(",")), tag!(",")) >>
        subs: terminated!(ws!(node_list), tag!(")")) >>
        ((target, to_str(flags), subs))
    )
);

named!(
    if_exp<(Node, Node, Node)>,
    do_parse!(
        tag!("IfExp") >>
        test: delimited!(tag!("("), node, tag!(",")) >>
        then: terminated!(ws!(node), tag!(",")) >>
        els: terminated!(ws!(node), tag!(")")) >>
        ((test, then, els))
    )
);

named!(
    string_literal<&str>,
    do_parse!(
        s: delimited!(tag!("'"), is_not!("'"), tag!("'")) >>
        (to_str(s))
    )
);

/*
named!(arg_list<Vec<Value>>, separated_list_complete!(tag!(","), ws!(arg)));
named!(arg<Value>, alt!(literal | map!(node, Value::Node)));
named!(literal<Value>, alt!(map!(none, |_| Value::None)));
named!(none, tag!("None"));

named!(const, 
*/

fn to_str(b: &[u8]) -> &str {
    str::from_utf8(b).unwrap()
}

pub fn parse_repr<'repr>(repr: &'repr [u8]) -> Result<Node<'repr>> {
    let parsed = match module(repr) {
        IResult::Done(remaining, parsed) => Node::Module(parsed.0, Box::new(parsed.1)),
        IResult::Error(e) => return Err(e.clone()).chain_err(|| {
            let source = to_str(repr);
            format!("Unable to parse module from source:\n{}", source)
        }),
        IResult::Incomplete(needed) => {
            return Err(
                ErrorKind::Msg(format!("Incomplete input, needed: {:?}", needed)).into(),
            )
        }
    };
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
