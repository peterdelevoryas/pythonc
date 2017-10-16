//!
//! Currently, this crate makes a ton of simplifications around
//! representing the IA-32 instruction set by assuming 32-bit
//! operands and instruction variants for everything. This is
//! further emphasized by (somewhat annoyingly) annotating
//! all types with "32".
//!

extern crate python_ir as ir;

pub mod register;
pub mod memory;
pub mod immediate;
pub mod ia_32;
pub mod att;

pub use register::Register;
pub use memory::Memory;
pub use memory::ScaleFactor;
pub use memory::Displacement;
pub use immediate::Immediate;
pub use att::Att;

use std::mem;
use ia_32::*;

pub struct Program {
    instructions: Vec<ia_32::Instruction>,
    stack_len: usize,
}

impl Program {
    fn new(stack_len: usize) -> Self {
        Self {
            instructions: Vec::new(),
            stack_len,
        }
    }

    /// Computes length of stack (size in bytes of stack = len * 4)
    fn compute_stack_len(ir: &ir::Program) -> usize {
        let mut len = 0;
        for stmt in &ir.stmts {
            if let &ir::Stmt::Def(_, _) = stmt {
                len += 1
            }
        }
        len
    }

    fn stack_location(&self, tmp: ir::Tmp) -> Memory {
        // 1 + index because we have to skip
        // the ebp pushed on the stack in the assumed shim
        let offset = ((tmp.index + 1) * mem::size_of::<u32>()) as isize * -1;
        let m = Memory {
            base: Register::EBP,
            index: None,
            displacement: Displacement(offset as i32),
        };
        m
    }

    pub fn build(ir: &ir::Program) -> Program {
        let stack_len = Self::compute_stack_len(ir);
        let mut program = Self::new(stack_len);
        for stmt in &ir.stmts {
            program.trans(stmt);
        }
        program
    }

    fn trans(&mut self, stmt: &ir::Stmt) {
        use ir::Stmt::*;
        use ir::Val::*;
        use ir::Expr;
        match *stmt {
            Print(val) => {
                let value = self.val_to_value(val, Register::EDI);
                // push value onto stack (x86 call argument)
                self.push(value);
                self.call("print_int_nl");
                // reset stack pointer
                self.add_esp_4();
            }
            Def(tmp, Expr::UnaryNeg(val)) => {
                let value = self.val_to_value(val, Register::EDI);
                let value = self.negate(value);
                let dst = self.stack_location(tmp);
                self.store(value, dst);
            }
            Def(tmp, Expr::Add(left, right)) => {
                // Be careful not to load tmps into same place here!!
                // TODO somehow make it clean and simple to perform
                // optimization of using EDI for right when left is value
                let left = self.val_to_value(left, Register::EDI);
                let right = self.val_to_value(right, Register::EDX);
                let sum = self.add(left, right);
                self.store_tmp(tmp, sum);
            }
            Def(tmp, Expr::FunCall(ref label, ref _args)) if label == "input" => {
                self.call("input");
                self.store_tmp(tmp, Value::Register(Register::EAX));
            }
            _ => unimplemented!(),
        }
    }

    fn val_to_value(&mut self, val: ir::Val, tmp_storage: Register) -> Value {
        use ir::Val::*;
        unimplemented!()
        /*
        match val {
            Ref(tmp) => {
                self.load_tmp(tmp, tmp_storage);
                Value::Register(tmp_storage)
            }
            Int(int) => Value::Immediate(Immediate(int)),
        }
        */
    }

    // Adds left to right, stores in dst if necessary
    fn add(&mut self, left: Value, right: Value) -> Value {
        let (dst, value) = match (left, right) {
            (Value::Immediate(Immediate(l)), Value::Immediate(Immediate(r))) => {
                return Value::Immediate(Immediate(l + r))
            }
            (Value::Register(l), r) => (l, r),
            (l, Value::Register(r)) => (r, l),
        };
        self.push_instruction(Instruction::Add(Add {
            value,
            register: dst,
        }));
        Value::Register(dst)
    }

    /// If register, negate the register, otherwise just change the constant value!
    fn negate(&mut self, value: Value) -> Value {
        match value {
            Value::Register(register) => {
                self.push_instruction(Instruction::Neg(Neg { register }));
                Value::Register(register)
            }
            Value::Immediate(Immediate(i)) => {
                // Statically computing negation on constants!
                Value::Immediate(Immediate(-i))
            }
        }
    }

    fn store_tmp(&mut self, tmp: ir::Tmp, value: Value) {
        let mem = self.stack_location(tmp);
        self.store(value, mem);
    }

    fn store(&mut self, value: Value, memory: Memory) {
        self.push_instruction(Instruction::Store(Store { value, memory }));
    }

    fn load_tmp(&mut self, tmp: ir::Tmp, register: Register) {
        let mem = self.stack_location(tmp);
        self.load(mem, register);
    }

    fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn add_esp_4(&mut self) {
        self.push_instruction(Instruction::Add(Add {
            value: Value::Immediate(Immediate(4)),
            register: Register::ESP,
        }));
    }

    fn load(&mut self, memory: Memory, register: Register) {
        self.push_instruction(Instruction::Load(Load { memory, register }));
    }

    fn push(&mut self, value: ia_32::Value) {
        self.push_instruction(Instruction::Push(Push { value }));
    }

    fn call<S: Into<String>>(&mut self, label: S) {
        let label = label.into();
        self.push_instruction(Instruction::Call(Call { label }));
    }
}

/*
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
    stack: Vec<ia32::Instr>,
    tmp_count: usize,
}

use self::mem::Mem;
use self::reg::Reg32;
use self::reg::EBP;
use self::reg::ESP;
use self::reg::EAX;
use self::ia32::{Push, Mov, Add, Neg, Call};

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
        let mut program: String = format!(
            "\
.globl main
main:
    pushl %ebp
    movl %esp, %ebp
    subl ${}, %esp

",
            self.tmp_count * Bits32::SIZE_OF
        );
        for ia32 in self.stack {
            let s = ia32.trans();
            program.push_str("    ");
            program.push_str(&s);
            program.push_str("\n");
        }
        program.push_str(
            "
    movl $0, %eax
    leave
    ret
",
        );
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
*/
