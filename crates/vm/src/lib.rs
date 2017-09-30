extern crate python_ir as ir;
extern crate python_trans as trans;

use std::fmt;
use trans::Att;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum RVal {
    Int(i32),
    LVal(LVal),
}


#[derive(Debug, Copy, Clone)]
pub enum LVal {
    Tmp(ir::Tmp),
    Register(trans::Register),
    Stack(usize),
}

#[derive(Debug, Clone)]
pub enum Instr {
    Mov(RVal, LVal),
    Neg(LVal),
    Add(RVal, LVal),
    Push(RVal),
    Call(String),
}

impl From<ir::Val> for RVal {
    fn from(v: ir::Val) -> Self {
        match v {
            ir::Val::Int(i) => RVal::Int(i),
            ir::Val::Ref(t) => RVal::LVal(LVal::Tmp(t)),
        }
    }
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RVal::*;
        match *self {
            Int(i) => write!(f, "${}", i),
            LVal(lval) => write!(f, "{}", lval),
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instr::*;
        match *self {
            Mov(rval, lval) => write!(f, "movl {}, {}", rval, lval),
            Neg(lval) => write!(f, "negl {}", lval),
            Add(rval, lval) => write!(f, "addl {}, {}", rval, lval),
            Push(rval) => write!(f, "pushl {}", rval),
            Call(ref s) => write!(f, "call {}", s),
        }
    }
}

impl Instr {
    pub fn replace_with(&mut self, tmp: ir::Tmp, new: LVal) {
        use self::Instr::*;
        use self::LVal::*;
        use self::RVal::*;
        match *self {
            Mov(Int(_), ref mut lval) => lval.replace_with(tmp, new),
            Mov(LVal(ref mut l), ref mut r) => {
                l.replace_with(tmp, new);
                r.replace_with(tmp, new);
            }
            Neg(ref mut lval) => lval.replace_with(tmp, new),
            Add(Int(_), ref mut lval) => lval.replace_with(tmp, new),
            Add(LVal(ref mut l), ref mut r) => {
                l.replace_with(tmp, new);
                r.replace_with(tmp, new);
            }
            Push(Int(_)) => {},
            Push(LVal(ref mut lval)) => lval.replace_with(tmp, new),
            Call(_) => {},
        }
    }

    pub fn replace_with_stack(&mut self, tmp: ir::Tmp, stack_index: usize) {
        self.replace_with(tmp, LVal::Stack(stack_index));
    }

    pub fn tmps(&self) -> HashSet<ir::Tmp> {
        use self::Instr::*;
        use self::LVal::*;
        use self::RVal::*;

        fn union(lhs: HashSet<ir::Tmp>, rhs: HashSet<ir::Tmp>) -> HashSet<ir::Tmp> {
            lhs.union(&rhs).map(|&v| v).collect()
        }

        match *self {
            Mov(LVal(left), right) => union(left.tmp(), right.tmp()),
            Neg(lval) => lval.tmp(),
            Add(LVal(left), right) => union(left.tmp(), right.tmp()),
            Push(LVal(lval)) => lval.tmp(),
            _ => HashSet::new(),
        }
    }
}

impl LVal {
    /// If this LVal is a Tmp, it is replaced with the new value,
    /// otherwise it is not modified
    pub fn replace_with(&mut self, tmp: ir::Tmp, new: LVal) {
        use self::LVal::*;
        use std::mem;
        match *self {
            Tmp(t) if t == tmp => {
                mem::replace(self, new);
            }
            _ => {}
        }
    }

    /// TODO replace with stuff
    fn tmp(&self) -> HashSet<ir::Tmp> {
        use self::LVal::*;
        match *self {
            Tmp(t) => {
                let mut set = HashSet::new();
                set.insert(t);
                set
            }
            _ => HashSet::new(),
        }
    }
}

pub struct Program {
    pub stack: Vec<Instr>,
    pub stack_index: usize,
}

impl Program {
    fn increment_stack_index(&mut self) {
        self.stack_index += 1;
    }

    pub fn build(ir: &ir::Program) -> Program {
        let mut program = Program { stack: vec![], stack_index: 0, };
        for stmt in &ir.stmts {
            program.trans(stmt);
        }
        program
    }

    pub fn print(&self) {
        for instr in &self.stack {
            println!("{}", instr);
        }
    }

    pub fn spill(&mut self, tmp: ir::Tmp) {
        for instr in self.stack.iter_mut() {
            // If the instruction doesn't reference tmp, then
            // this won't modify the instruction
            instr.replace_with_stack(tmp, self.stack_index);
        }
        self.increment_stack_index();
    }

    /// Fixes up mov stack, stack and add stack, stack
    pub fn replace_stack_to_stack_ops(&self, alloc: &mut ir::TmpAllocator) -> Program {
        use self::Instr::*;
        use self::LVal::*;
        use self::RVal::*;
        let mut fixed = Program { stack: vec![], stack_index: 0, };
        for instr in &self.stack {
            match *instr {
                Mov(LVal(Stack(left)), Stack(right)) => {
                    let tmp = alloc.alloc().expect("tmp allocation error");
                    let mov_to_tmp = Mov(LVal(Stack(left)), Tmp(tmp));
                    let mov_from_tmp = Mov(LVal(Tmp(tmp)), Stack(right));
                    fixed.stack.push(mov_to_tmp);
                    fixed.stack.push(mov_from_tmp);
                }
                Add(LVal(Stack(left)), Stack(right)) => {
                    let tmp = alloc.alloc().expect("tmp allocation error");
                    let mov_to_tmp = Mov(LVal(Stack(right)), Tmp(tmp));
                    let add_to_tmp = Add(LVal(Stack(left)), Tmp(tmp));
                    fixed.stack.push(mov_to_tmp);
                    fixed.stack.push(add_to_tmp);
                }
                ref i => fixed.stack.push(i.clone()),
            }
        }

        fixed
    }

    ///
    /// ```
    /// tmp := l + r => {
    ///     mov l, tmp
    ///     add r, tmp
    /// }
    ///
    /// tmp := -v => {
    ///     mov v, tmp
    ///     neg tmp
    /// }
    ///
    /// tmp := input() => {
    ///     call input, eax // eax acts as destination operand implicitly,
    ///                     // this is hardcoded in liveness analysis
    ///     mov eax, tmp
    /// }
    ///
    /// print v => {
    ///     push v
    ///     call print_int_nl
    /// }
    /// ```
    ///
    fn trans(&mut self, stmt: &ir::Stmt) {
        use ir::Stmt::*;
        use ir::Expr::*;
        use ir::Val::*;
        match *stmt {
            Print(v) => {
                self.push(v.into());
                self.call("print_int_nl");
            }
            Def(tmp, Add(l, r)) => {
                self.mov(l.into(), LVal::Tmp(tmp));
                self.add(r.into(), LVal::Tmp(tmp));
            }
            Def(tmp, UnaryNeg(v)) => {
                self.mov(v.into(), LVal::Tmp(tmp));
                self.neg(LVal::Tmp(tmp));
            }
            Def(tmp, Input) => {
                self.call("input");
                let eax = RVal::LVal(LVal::Register(trans::Register::EAX));
                self.mov(eax, LVal::Tmp(tmp));
            }
        }
    }

    fn neg(&mut self, lval: LVal) {
        self.stack.push(Instr::Neg(lval));
    }

    fn add(&mut self, rval: RVal, lval: LVal) {
        self.stack.push(Instr::Add(rval, lval));
    }

    fn mov(&mut self, rval: RVal, lval: LVal) {
        self.stack.push(Instr::Mov(rval, lval));
    }

    fn push(&mut self, rval: RVal) {
        self.stack.push(Instr::Push(rval));
    }

    fn call(&mut self, s: &str) {
        self.stack.push(Instr::Call(s.into()));
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instr::*;
        use self::LVal::*;
        use self::RVal::*;
        write!(f,
"
.globl main
main:
    pushl %ebp
    movl %esp, %ebp
    subl ${}, %esp
",
            self.stack_index * 4)?;

        for instr in &self.stack {
            writeln!(f, "    {}", instr)?;
        }

        write!(f,
"
    movl $0, %eax
    leave
    ret
")?;
        Ok(())
    }
}

impl fmt::Display for LVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LVal::Tmp(tmp) => write!(f, "{}", tmp),
            LVal::Register(r) => write!(f, "{}", trans::Att(&r)),
            LVal::Stack(index) => {
                let offset = (index as i32 + 1) * -4;
                let mem = trans::Memory {
                    base: trans::Register::EBP,
                    index: None,
                    displacement: trans::Displacement(offset as i32),
                };
                write!(f, "{}", trans::Att(&mem))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
