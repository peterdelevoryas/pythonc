extern crate rust_python;

use rust_python::lexer;
use rust_python::p0;
use rust_python::ir;

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
    let statements = p0::parse_statements(source, lexer).unwrap();

    let mut ir = ir::Builder::new();
    for statement in statements {
        ir.flatten_statement(&statement);
    }

    println!("\nintermediate representation:");
    for (i, stmt) in ir.stack().iter().enumerate() {
        let line = match *stmt {
            ir::Stmt::Print(ref val) => format!("print {}", val_to_string(val)),
            ir::Stmt::Def(ref tmp, ref expr) => {
                let tmp = val_to_string(&ir::Val::Ref(*tmp));
                match *expr {
                    ir::Expr::UnaryNeg(ref val) => format!("{:<3} := -{}", tmp, val_to_string(val)),
                    ir::Expr::Add(ref l, ref r) => format!("{:<3} := {} + {}", tmp, val_to_string(l), val_to_string(r)),
                    ir::Expr::Input => format!("{:<3} := input()", tmp),
                }
            }
        };
        println!(" {:<4} {}", i, line);
    }
}
