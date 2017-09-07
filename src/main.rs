extern crate rust_python;

use rust_python::lexer;
use rust_python::p0;
use rust_python::ir;
use rust_python::x86;

use std::env;
use std::fs;
use std::io::Read;

fn val_to_string(val: &ir::Val) -> String {
    match *val {
        ir::Val::Int(i) => format!("{}", i),
        ir::Val::Ref(tmp) => format!("t{}", tmp.index()),
    }
}

fn main() {
    let source = {
        let path = env::args().nth(1).unwrap();
        let mut f = fs::File::open(&path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        buf
    };
    let source = source.as_str();

    println!("source:");
    for (i, line) in source.lines().enumerate() {
        println!(" {:<4} {}", i, line);
    }

    let lexer = lexer::Lexer::new(source);
    let ast = p0::parse_program(source, lexer).unwrap();
    let ir: ir::Program = ast.into();
    let x86 = x86::Builder::build(&ir);
    println!("x86:\n{}", x86);
}
