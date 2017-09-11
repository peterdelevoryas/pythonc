extern crate pythonc_token as token;
extern crate pythonc_ast as ast;
extern crate pythonc_ir as ir;
extern crate pythonc_trans as trans;
#[macro_use]
extern crate error_chain;

pub mod error;

pub use error::{Error, ErrorKind, Result, ResultExt};

pub fn compile(source: &str) -> Result<String> {
    let mut tokens = token::Stream::new(source);
    let ast = ast::parse_program(tokens).chain_err(|| "parse error")?;
    let ir = ast.into();
    let asm = trans::Builder::build(&ir);
    Ok(asm)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
