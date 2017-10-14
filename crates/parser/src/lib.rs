extern crate python_ast as ast;

pub fn parse_program(s: &str) -> Result<ast::Program, String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
