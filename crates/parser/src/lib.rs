extern crate python_ast as ast;

use std::process::Command;

// Takes the repr(python.compiler.parse(source)) as input
pub fn parse_program(s: &str) -> Result<ast::Program, String> {
    println!("received: {}", s);
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
