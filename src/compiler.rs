use lexer;
use p0;
use ast;
use ir;
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
        let ast = self.build_ast(source)?;
        let ir = self.build_ir(&ast)?;
        self.build_asm(&ir)
    }

    pub fn build_ast(&self, source: &str) -> Result<ast::Program> {
        let lexer = lexer::Lexer::new(source);
        p0::parse_program(lexer).chain_err(|| "invalid program")
    }

    pub fn build_ir(&self, ast: &ast::Program) -> Result<ir::Program> {
        let ir = ast.into();
        Ok(ir)
    }

    pub fn build_asm(&self, ir: &ir::Program) -> Result<String> {
        let asm = x86::Builder::build(ir);
        Ok(asm)
    }
}
