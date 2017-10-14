extern crate python_ast as ast;
#[macro_use]
extern crate nom;

use nom::IResult::Done;

use std::process::Command;
use std::str;
use std::fmt;

#[derive(Debug)]
struct Node<'s> {
    name: &'s str,
    args: Vec<Value<'s>>,
}

#[derive(Debug)]
enum Value<'s> {
    None,
    Node(Node<'s>),
}

named!(node<Node>,
    do_parse!(
        name: node_name >> 
        args: delimited!(tag!("("), arg_list, tag!(")")) >>
        (Node {
            name: to_str(name),
            args: args,
        })
    )
);

named!(arg_list<Vec<Value>>, separated_list_complete!(tag!(","), ws!(arg)));
named!(arg<Value>, alt!(literal | map!(node, Value::Node)));
named!(node_name, alt!(tag!("Module")   |
                       tag!("Stmt")     |
                       tag!("Discard")  |
                       tag!("Const")    |
                       tag!("UnarySub") |
                       tag!("Printnl")));

named!(literal<Value>, alt!(map!(none, |_| Value::None)));
named!(none, tag!("None"));

fn to_str(b: &[u8]) -> &str {
    str::from_utf8(b).unwrap()
}

// Takes the repr(python.compiler.parse(source)) as input
pub fn parse_program(s: &[u8]) -> Result<ast::Program, String> {
    //println!("received: {}", str::from_utf8(s).unwrap());
    let parsed = node(b"Module(None, Module(None))").unwrap();
    println!("parsed: {:#?}", parsed.1);
    println!("remaining: {:?}", to_str(parsed.0));

    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
