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

#[derive(Debug, Copy, Clone)]
pub enum Bits8 {}
impl Sealed for Bits8 {}
impl Bits for Bits8 {
    const SIZE_OF: usize = 1;
}

#[derive(Debug, Copy, Clone)]
pub enum Bits16 {}
impl Sealed for Bits16 {}
impl Bits for Bits16 {
    const SIZE_OF: usize = 2;
}

#[derive(Debug, Copy, Clone)]
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
use self::reg::EAX;
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
                self.push(Push::Imm(int));
                // call print
                self.call("print_int_nl");
                // reset stack pointer
                self.add(Add::ImmReg(4, ESP));
            }
            Print(Ref(tmp)) => {
                // location of tmp on stack
                let mem = self.stack_location(tmp);
                // push tmp's value onto stack
                self.push(Push::Mem(mem));
                // call print
                self.call("print_int_nl");
                // reset stack pointer
                self.add(Add::ImmReg(4, ESP));
            }
            Def(tmp, Expr::UnaryNeg(val)) => {
                let mem = self.stack_location(tmp);
                self.store_val(val, mem);
                // we can just negate the memory location! (after storing)
                self.neg(Neg::Mem(mem));
            }
            Def(tmp, Expr::Add(left, right)) => {
                let mem = self.stack_location(tmp);
                self.store_val(left, mem);
                self.add_val(right, mem);
            }
            Def(tmp, Expr::Input) => {
                let dst = self.stack_location(tmp);
                self.call("input");
                self.store_reg(EAX, dst);
            }
        }
    }

    fn finish(self) -> String {
        let mut program: String = format!("\
.globl main
main:
    pushl %ebp
    movl %esp, %ebp
    subl ${}, %esp

", self.tmp_count * Bits32::SIZE_OF);
        for ia32 in self.stack {
            let s = ia32.trans();
            program.push_str("    ");
            program.push_str(&s);
            program.push_str("\n");
        }
        program.push_str("
    movl $0, %eax
    leave
    ret
"       );
        program
    }

    fn load(&mut self, mem: Mem<Bits32, Reg32, i32>, reg: Reg32) {
        self.stack.push(Box::new(Mov::MemReg(mem, reg)));
    }

    fn store_reg(&mut self, reg: Reg32, dst: Mem<Bits32, Reg32, i32>) {
        self.stack.push(Box::new(Mov::RegMem(reg, dst)));
    }

    fn store_val(&mut self, val: ir::Val, dst: Mem<Bits32, Reg32, i32>) {
        match val {
            ir::Val::Int(int) => {
                self.stack.push(Box::new(Mov::ImmMem(int, dst)));
            }
            ir::Val::Ref(tmp) => {
                // basically just assume EAX is ok to use
                let src = self.stack_location(tmp);
                self.load(src, EAX);
                self.store_reg(EAX, dst);
            }
        }
    }

    fn neg(&mut self, neg: Neg<Bits32, Reg32, i32>) {
        self.stack.push(Box::new(neg));
    }

    fn add_val(&mut self, val: ir::Val, dst: Mem<Bits32, Reg32, i32>) {
        match val {
            ir::Val::Int(int) => {
                self.add(Add::ImmMem(int, dst));
            }
            ir::Val::Ref(tmp) => {
                let src = self.stack_location(tmp);
                self.load(src, EAX);
                self.add(Add::RegMem(EAX, dst));
            }
        }
    }

    fn add(&mut self, add: Add<Bits32, Reg32, i32>) {
        self.stack.push(Box::new(add));
    }

    fn push(&mut self, push: Push<Reg32, i32>) {
        self.stack.push(Box::new(push));
    }

    fn call(&mut self, label: &str) {
        self.stack.push(Box::new(Call { label: label.into() }));
    }
}
