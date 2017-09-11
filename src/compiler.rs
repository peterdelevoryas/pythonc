use tok;
use ast::Program as Ast;
use ast;
use ir::Program as Ir;
use x86;
use error::Result;
use error::ResultExt;

/// 
/// ```rust
/// # use error::Result;
/// # fn try_main() -> Result<()> {
///       let program = "
///           x = 1
///           y = 2
///           print x + y
///       ";
///       let compiler = Compiler::new();
///       let ast = compiler.build_ast(source)?;
///       let ir = compiler.build_ir(&ast)?;
///       let asm = compiler.build_asm(&ir)?;
///       let asm = compiler.compile(source)?;
/// #     Ok(())
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
        let tok = self.tok_stream(source);
        let ast = self.parse_ast(tok)?;
        let ir = self.build_ir(&ast)?;
        self.build_asm(&ir)
    }

    pub fn tok_stream<'source>(&self, source: &'source str) -> tok::Stream<'source> {
        tok::Stream::new(source)
    }

    pub fn parse_ast<I>(&self, tok: I) -> Result<Ast>
        where I: Iterator<Item=tok::Spanned<tok::Tok>>,
    {
        let mut parser = ast::Parser::new(tok);
        parser.parse_program().chain_err(|| "invalid program")
    }

    pub fn build_ir(&self, ast: &Ast) -> Result<Ir> {
        let ir = ast.into();
        Ok(ir)
    }

    pub fn build_asm(&self, ir: &Ir) -> Result<String> {
        let asm = x86::Builder::build(ir);
        Ok(asm)
    }
}
