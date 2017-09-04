extern crate rust_python;

use rust_python::lexer;
use rust_python::p0;
use rust_python::ir;

fn val_to_string(val: &ir::Val) -> String {
    match *val {
        ir::Val::Int(i) => format!("{}", i),
        ir::Val::Ref(tmp) => format!("t{}", tmp.index()),
    }
}

fn main() {
    let source = "print 1 + 2
rust_python = 22 + -input()
print rust_python + -2 + 1 + input()
x = 1
y = 2 + input() + x
x = 2
999 + 0
print x + y
";
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
                    ir::Expr::Copy(ref val) => format!("{:<3} := {}", tmp, val_to_string(val)),
                    ir::Expr::UnaryNeg(ref val) => format!("{:<3} := -{}", tmp, val_to_string(val)),
                    ir::Expr::Add(ref l, ref r) => format!("{:<3} := {} + {}", tmp, val_to_string(l), val_to_string(r)),
                    ir::Expr::Input => format!("{:<3} := input()", tmp),
                }
            }
        };
        println!(" {:<4} {}", i, line);
    }
}
