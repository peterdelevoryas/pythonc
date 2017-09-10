use lexer;
use p0;
use ast;
use ir;
use x86;
use error::Result;
use error::ResultExt;
use std::iter::IntoIterator;

/// 
/// ```rust
/// # use error::Result;
/// # fn try_main() -> Result<()> {
///       let program = "
///           x = 1
///           y = 2
///           print x + y
///       ";
///       let asm: String = Compiler::new().compile(program)?;
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
#[derive(Debug)]
pub struct Compiler {
    private: ()
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { private: () }
    }

    pub fn compile(&self, source: &str) -> Result<String> {
        let lexer = lexer::Lexer::new(source);
        let ast = p0::parse_program(lexer).chain_err(|| "invalid program")?;
        let ir = ast.into();
        let x86 = x86::Builder::build(&ir);
        Ok(x86)
    }
}
