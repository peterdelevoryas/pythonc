use std::io;
use vm::Visit;
use vm::Module;
use vm::BlockData;
use vm::FuncData;
use vm::Inst;
use vm::InstData;
use vm::Term;
use vm::Reg;
use vm;

struct Writer<'dst, W>
where
    W: io::Write + 'dst,
{
    dst: &'dst mut W,
}

impl<'dst, W> Visit for Writer<'dst, W>
where
    W: io::Write + 'dst
{
    fn visit_module(&mut self, module: &Module) {
        writeln!(self.dst, ".globl main").unwrap();
        self.traverse_module(module);
    }

    fn visit_func(&mut self, func: &FuncData) {
        writeln!(self.dst, "\
{func}:
    pushl %ebx
    pushl %edi
    pushl %esi
    pushl %ebp
    movl %esp, %ebp
    subl ${stack_size}, %esp", func=func.name(), stack_size=func.stack_slots * 4).unwrap();

        {
            let mut func_writer = FuncWriter {
                dst: self.dst,
                func: func,
            };
            func_writer.traverse_func(func);
        }

        writeln!(self.dst, "\
{func}.ret:
    movl %ebp, %esp
    popl %ebp
    popl %esi
    popl %edi
    popl %ebx
    ret", func=func.name()).unwrap();
    
    }
}

pub fn write_gas<W>(dst: &mut W, module: &vm::Module)
where
    W: io::Write,
{
    let mut writer = Writer { dst };
    writer.visit_module(module);
}

struct FuncWriter<'dst, 'func, W>
where
    W: io::Write + 'dst,
{
    dst: &'dst mut W,
    func: &'func FuncData,
}

impl<'dst, 'func, W> Visit for FuncWriter<'dst, 'func, W>
where
    W: io::Write + 'dst,
{
    fn visit_block(&mut self, block: &BlockData) {
        writeln!(self.dst, "{}.{}:", self.func.name(), block.name()).unwrap();
        self.traverse_block(block);
    }

    fn visit_inst(&mut self, inst: &Inst) {
        use self::InstData::*;
        match inst.data {
            Unary { opcode, ref arg } => {
                writeln!(self.dst, "    movl {}, {}", arg, inst.dst).unwrap();
                match opcode {
                    // Don't need to do anything else for mov's
                    vm::Unary::Mov => {}
                    vm::Unary::Neg | vm::Unary::Not => writeln!(self.dst, "    {} {}", opcode, inst.dst).unwrap(),
                    vm::Unary::Push => panic!("XXX Remove Unary::Push!"),
                }
            }
            Binary { opcode, ref left, ref right } => {
                match opcode {
                    vm::Binary::Add |
                    vm::Binary::Sub |
                    vm::Binary::Or |
                    vm::Binary::And |
                    vm::Binary::Shr |
                    vm::Binary::Shl => {
                        if let vm::Rval::Lval(ref r) = *right {
                            if r == &inst.dst {
                                println!("right == dst: {}", inst);
                                writeln!(self.dst, "    movl {}, {}", right, inst.dst).unwrap();
                                writeln!(self.dst, "    {} {}, {}", opcode, left, inst.dst).unwrap();
                                return;
                            }
                        }
                        writeln!(self.dst, "    movl {}, {}", left, inst.dst).unwrap();
                        writeln!(self.dst, "    {} {}, {}", opcode, right, inst.dst).unwrap();
                    }
                    vm::Binary::Sete |
                    vm::Binary::Setne => {
                        use vm::Reg::*;
                        writeln!(self.dst, "    cmpl {}, {}", left, right).unwrap();
                        writeln!(self.dst, "    movl $0, {}", inst.dst).unwrap();
                        let dst = match inst.dst {
                            vm::Lval::Var(_) => panic!(),
                            vm::Lval::StackSlot(s) => format!("{}", s),
                            vm::Lval::Reg(r) => {
                                match r {
                                    EAX => "%al",
                                    ECX => "%cl",
                                    EDX => "%dl",
                                    EBX => "%bl",
                                    _ => panic!()
                                }.into()
                            }
                        };
                        writeln!(self.dst, "    {} {}", opcode, dst).unwrap();
                    }
                }
            }
            CallIndirect { ref target, ref args } => {
                let args_size = args.len() * 4;
                for arg in args.iter().rev() {
                    writeln!(self.dst, "    pushl {}", arg).unwrap();
                }
                writeln!(self.dst, "    call *{}", target).unwrap();
                writeln!(self.dst, "    addl ${}, %esp", args_size).unwrap();
                writeln!(self.dst, "    movl %eax, {}", inst.dst).unwrap();
            }
            Call { ref func, ref args } => {
                let args_size = args.len() * 4;
                for arg in args.iter().rev() {
                    writeln!(self.dst, "    pushl {}", arg).unwrap();
                }
                writeln!(self.dst, "    call {}", func).unwrap();
                writeln!(self.dst, "    addl ${}, %esp", args_size).unwrap();
                writeln!(self.dst, "    movl %eax, {}", inst.dst).unwrap();
            }
            ShiftLeftThenOr { ref arg, shift, or } => {
                writeln!(self.dst, "    movl {}, {}", arg, inst.dst).unwrap();
                writeln!(self.dst, "    shll ${}, {}", shift, inst.dst).unwrap();
                writeln!(self.dst, "    orl ${}, {}", or, inst.dst).unwrap();
            }
            MovFuncLabel { ref func } => {
                writeln!(self.dst, "    movl ${}, {}", func, inst.dst).unwrap();
            }
            Phi { ref lvals } => {
                println!("WARNING: phi node in assembly output");
                writeln!(self.dst, "    phi({})", ::itertools::join(lvals, ", ")).unwrap();
            }
        }
    }

    fn visit_term(&mut self, term: &Term) {
        use self::Term::*;
        match *term {
            Return { ref rval } => {
                if let Some(ref rval) = *rval {
                    writeln!(self.dst, "    movl {}, %eax", rval).unwrap();
                }
                writeln!(self.dst,
"    jmp {func}.ret",
                func=self.func.name()).unwrap();
            }
            Goto { ref block } => {
                writeln!(self.dst,
"    jmp {func}.{block}",
                func=self.func.name(), block=self.func.block(block).name()).unwrap();
            }
            Switch { ref cond, ref then, ref else_ } => {
                writeln!(self.dst, "    cmpl $0, {cond}\n    jnz {func}.{then}\n    jmp {func}.{else_}",
                cond=cond,
                func=self.func.name(),
                then=self.func.block(then).name(),
                else_=self.func.block(else_).name()).unwrap();
            }
        }
    }
}
