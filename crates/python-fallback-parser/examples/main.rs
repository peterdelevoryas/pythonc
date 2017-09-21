extern crate python_fallback_parser as parser;

use parser::*;

fn main() {
    let p = "1 + 2\ninput()\nprint 3 + input()\nx = input()\ny=x";
    println!("{:?}", parse_program_fallback(p).unwrap());
}