extern crate python_ast as ast;
#[macro_use]
extern crate nom;

use nom::IResult::*;

use std::process::Command;
use std::str;
use std::fmt;

pub type BoxNode<'a> = Box<Node<'a>>;

#[derive(Debug)]
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
    AssName(&'a str, &'a str),
    // Add((left, right))
    Add(BoxNode<'a>, BoxNode<'a>),
    // UnarySub(expr)
    UnarySub(BoxNode<'a>),
    // CallFUnc(node, args)
    CallFunc(BoxNode<'a>, Vec<Node<'a>>),
}

named!(
    node<Node>,
    alt!(
        map!(module, |(doc, node)| Node::Module(doc, Box::new(node)))   |
        map!(stmt, |nodes| Node::Stmt(nodes))                           |
        map!(constant, |val| Node::Const(val))                          |
        map!(discard, |node| Node::Discard(Box::new(node)))             |
        map!(name, |name| Node::Name(name))                             |
        map!(printnl, |(nodes, dest)| Node::Printnl(nodes, dest))
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

named!(node_list<Vec<Node>>, delimited!(tag!("["), separated_list_complete!(tag!(","), ws!(node)), tag!("]")));

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
        name: delimited!(tag!("("), is_not!(")"), tag!(")")) >>
        (to_str(name))
    )
);

named!(
    printnl<(Vec<Node>, &str)>,
    do_parse!(
        tag!("Printnl") >>
        nodes: delimited!(tag!("("), node_list, tag!(",")) >>
        dest: ws!(is_not!(")")) >>
        tag!(")") >>
        (nodes, to_str(dest))
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

// Takes the repr(python.compiler.parse(source)) as input
pub fn parse_program(s: &[u8]) -> Result<ast::Program, String> {
    //println!("received: {}", str::from_utf8(s).unwrap());
    match node(s) {
        Done(remaining, parsed) => {
            println!("remaining: {}", str::from_utf8(remaining).unwrap());
            println!("parsed: {:#?}", parsed);
        }
        Error(e) => panic!("Error: {}", e),
        Incomplete(s) => panic!("Incomplete: {:?}", s),
    }

    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
