pub mod reg;
pub mod imm;
pub mod ia32;
pub mod mem;

mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

pub trait Bits: Sealed {
    const SIZE_OF: usize;
}

#[derive(Debug)]
pub enum Bits8 {}
impl Sealed for Bits8 {}
impl Bits for Bits8 {
    const SIZE_OF: usize = 1;
}

#[derive(Debug)]
pub enum Bits16 {}
impl Sealed for Bits16 {}
impl Bits for Bits16 {
    const SIZE_OF: usize = 2;
}

#[derive(Debug)]
pub enum Bits32 {}
impl Sealed for Bits32 {}
impl Bits for Bits32 {
    const SIZE_OF: usize = 4;
}

pub struct Builder {
    stack: Vec<Box<ia32::Instr>>,
    tmp_count: usize,
}

use ir;
use self::mem::Mem;
use self::imm::Imm;
use self::reg::Reg;
use self::reg::Reg32;
use self::reg::EBP;
use self::reg::ESP;
use self::ia32::{
    Push,
    Mov,
    Add,
    Neg,
    Ret,
    Call,
};

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

    fn stack_location(&self, tmp: ir::Tmp) -> Mem<Bits32, Reg32, i32> {
        // 1 + index because we have to skip
        // the ebp pushed on the stack in the assumed shim
        let offset = ((tmp.index + 1) * Bits32::SIZE_OF) as isize * -1;
        let m = Mem {
            base: EBP,
            disp: offset as i32,
        };
        m
    }

    fn trans(&mut self, stmt: &ir::Stmt) {
        use ir::Stmt::*;
        use ir::Val::*;
        use ir::Expr;
        match *stmt {
            Print(Int(int)) => {
                // push arg onto stack
                self.push::<Reg32, i32>(Push::Imm(int));
                // call print
                self.call("print_int_nl");
                // reset stack pointer
                self.add(Add::ImmReg(4, ESP));
            }
            Print(Ref(tmp)) => {
                // location of tmp on stack
                let mem = self.stack_location(tmp);
                // push tmp's value onto stack
                self.push::<Reg32, i32>(Push::Mem(mem));
                // call print
                self.call("print_int_nl");
                // reset stack pointer
                self.add(Add::ImmReg(4, ESP));
            }
            Def(tmp, Expr::UnaryNeg(val)) => {

            }
            Def(tmp, Expr::Add(left, right)) => {

            }
            Def(tmp, Expr::Input) => {

            }
        }
    }

    fn finish(self) -> String {
        unimplemented!()
    }

    fn add<B, R, I>(&mut self, add: Add<B, R, I>)
        where B: 'static + Bits,
              R: 'static + Reg<Size=B>,
              I: 'static + Imm<Size=B>,
    {
        self.stack.push(Box::new(add));
    }

    fn push<R, I>(&mut self, push: Push<R, I>)
        where R: 'static + Reg<Size=Bits32>,
              I: 'static + Imm<Size=Bits32>,
    {
        self.stack.push(Box::new(push));
    }

    fn call(&mut self, label: &str) {
        self.stack.push(Box::new(Call { label: label.into() }));
    }
}
