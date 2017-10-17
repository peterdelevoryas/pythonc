use LValue;
use RValue;
use Instruction;
use ir;
use trans;
use std::fmt;

pub struct Program {
    pub stack: Vec<Instruction>,
    pub stack_index: usize,
}

impl Program {
    fn increment_stack_index(&mut self) {
        self.stack_index += 1;
    }

    pub fn build(ir: &ir::Program) -> Program {
        let mut program = Program {
            stack: vec![],
            stack_index: 0,
        };
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
        //println!("spilling into stack {}", self.stack_index);
        for instr in self.stack.iter_mut() {
            // If the instruction doesn't reference tmp, then
            // this won't modify the instruction
            instr.replace_with_stack(tmp, self.stack_index);
        }
        self.increment_stack_index();
    }

    /// Fixes up mov stack, stack and add stack, stack
    pub fn replace_stack_to_stack_ops(&self, alloc: &mut ir::TmpAllocator) -> Program {
        use self::Instruction::*;
        use self::LValue::*;
        use self::RValue::*;
        let mut fixed = Program {
            stack: vec![],
            stack_index: self.stack_index,
        };
        for instr in &self.stack {
            match *instr {
                Mov(LValue(Stack(left)), Stack(right)) => {
                    let tmp = alloc.alloc().expect("tmp allocation error");
                    let mov_to_tmp = Mov(LValue(Stack(left)), Tmp(tmp));
                    let mov_from_tmp = Mov(LValue(Tmp(tmp)), Stack(right));
                    fixed.stack.push(mov_to_tmp);
                    fixed.stack.push(mov_from_tmp);
                }
                Add(LValue(Stack(left)), Stack(right)) => {
                    let tmp = alloc.alloc().expect("tmp allocation error");
                    let mov_to_tmp = Mov(LValue(Stack(right)), Tmp(tmp));
                    let add_to_tmp = Add(LValue(Stack(left)), Tmp(tmp));
                    let store_tmp = Mov(LValue(Tmp(tmp)), Stack(right));
                    fixed.stack.push(mov_to_tmp);
                    fixed.stack.push(add_to_tmp);
                    fixed.stack.push(store_tmp);
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
                self.mov(l.into(), LValue::Tmp(tmp));
                self.add(r.into(), LValue::Tmp(tmp));
            }
            Def(tmp, UnaryNeg(v)) => {
                self.mov(v.into(), LValue::Tmp(tmp));
                self.neg(LValue::Tmp(tmp));
            }
            Def(tmp, Inject(int)) => {
                let rval = int.into();
                self.mov(rval, LValue::Tmp(tmp));
            }
            Def(tmp, FunCall(ref name, ref args)) => {
                // push args in reverse order
                for &arg in args.iter().rev() {
                    self.push(arg.into());
                }

                // call function
                self.call(name);

                // move eax into destination tmp value
                let eax = RValue::LValue(LValue::Register(trans::Register::EAX));
                self.mov(eax, LValue::Tmp(tmp));
            }
            Def(tmp, ref expr) => unimplemented!("unimplemented def in vm: {:?}", expr),
        }
    }

    fn neg(&mut self, lval: LValue) {
        self.stack.push(Instruction::Neg(lval));
    }

    fn add(&mut self, rval: RValue, lval: LValue) {
        self.stack.push(Instruction::Add(rval, lval));
    }

    fn mov(&mut self, rval: RValue, lval: LValue) {
        self.stack.push(Instruction::Mov(rval, lval));
    }

    fn push(&mut self, rval: RValue) {
        self.stack.push(Instruction::Push(rval));
    }

    fn call(&mut self, s: &str) {
        self.stack.push(Instruction::Call(s.into()));
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        use self::LValue::*;
        use self::RValue::*;
        write!(
            f,
            "
.globl main
main:
    pushl %ebp
    movl %esp, %ebp

    {}
",
            {
                let len = (self.stack_index + 1) * 4;
                if len != 0 {
                    format!("subl ${}, %esp", len)
                } else {
                    "".into()
                }
            }
        )?;

        for instr in &self.stack {
            writeln!(f, "    {}", instr)?;
        }

        write!(
            f,
            "
    movl $0, %eax
    leave
    ret
"
        )?;
        Ok(())
    }
}


