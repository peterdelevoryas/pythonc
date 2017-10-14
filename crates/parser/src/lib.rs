extern crate python_ast as ast;
#[macro_use]
extern crate nom;

use nom::IResult::*;

use std::process::Command;
use std::str;
use std::fmt;

type BoxNode<'a> = Box<Node<'a>>;

#[derive(Debug)]
enum Node<'a> {
    // Module(doc, node)
    Module(&'a str, BoxNode<'a>),
    Stmt(Vec<Node<'a>>),
    Const(&'a str),
    Discard(BoxNode<'a>),
}

named!(
    node<Node>,
    alt!(
        map!(module, |(doc, node)| Node::Module(to_str(doc), Box::new(node)))   |
        map!(stmt, |nodes| Node::Stmt(nodes))                                   |
        map!(discard, |node| Node::Discard(Box::new(node)))                     |
        map!(constant, |val| Node::Const(to_str(val)))
    )
);

named!(
    module<&[u8], (&[u8], Node)>,
    do_parse!(
        tag!("Module") >>
        doc: delimited!(tag!("("), is_not!(","), tag!(",")) >>
        node: ws!(node) >>
        tag!(")") >>
        ((doc, node))
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
    constant<&[u8]>,
    do_parse!(
        tag!("Const") >>
        v: delimited!(tag!("("), is_not!(")"), tag!(")")) >>
        (v)
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
