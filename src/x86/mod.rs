pub mod reg;
pub mod imm;
pub mod ia32;
pub mod mem;

mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

pub trait Bits: Sealed {}

#[derive(Debug)]
pub enum Bits8 {}
impl Sealed for Bits8 {}
impl Bits for Bits8 {}

#[derive(Debug)]
pub enum Bits16 {}
impl Sealed for Bits16 {}
impl Bits for Bits16 {}

#[derive(Debug)]
pub enum Bits32 {}
impl Sealed for Bits32 {}
impl Bits for Bits32 {}

pub struct Builder {
    stack: Vec<Box<ia32::Instr>>,
    tmp_count: usize,
}

use ir;

impl Builder {
    fn new(tmp_count: usize) -> Builder {
        Builder {
            stack: vec![],
            tmp_count,
        }
    }

    fn compute_tmp_count(program: &ir::Program) -> usize {
        let mut count = 0;
        for stmt in &program.stmts {
            if let &ir::Stmt::Def(_, _) = stmt {
                count += 1
            }
        }
        count
    }

    pub fn build(program: &ir::Program) -> String {
        let tmp_count = Self::compute_tmp_count(program);
        let mut builder = Builder::new(tmp_count);
        for stmt in &program.stmts {
            builder.trans(stmt);
        }
        builder.finish()
    }

    fn trans(&mut self, stmt: &ir::Stmt) {
        use ir::Stmt::*;
    }

    fn finish(self) -> String {
        unimplemented!()
    }
}
