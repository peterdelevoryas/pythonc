use std::fmt;
use ia_32::*;
use register::Register;
use memory::Memory;
use memory::ScaleFactor;
use memory::Displacement;
use immediate::Immediate;
use Program;

/// Wrapper struct used to implement
/// custom AT&T syntax formatting
pub struct Att<'a, T: 'a>(pub &'a T);

impl<'a> fmt::Display for Att<'a, Displacement> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Instead of writing $2, write 2 by itself
        // Also, apparently the Rust parser
        // can't handle self.0.0 without parens
        write!(f, "{}", &(self.0).0)
    }
}

impl<'a> fmt::Display for Att<'a, Immediate> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", (self.0).0)
    }
}

impl<'a> fmt::Display for Att<'a, Register> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.0.as_str())
    }
}

impl<'a> fmt::Display for Att<'a, ScaleFactor> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.value())
    }
}

impl<'a> fmt::Display for Att<'a, Memory> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.index {
            Some((index, scale_factor)) => {
                write!(f, "{}({}, {}, {})", Att(&self.0.displacement), Att(&self.0.base), Att(&index), Att(&scale_factor))
                    
            }
            None => {
                write!(f, "{}({})", Att(&self.0.displacement), Att(&self.0.base))
            }
        }
    }
}

impl<'a> fmt::Display for Att<'a, Value> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Value::Register(ref register) => write!(f, "{}", Att(register)),
            Value::Immediate(ref immediate) => write!(f, "{}", Att(immediate)),
        }
    }
}

impl<'a> fmt::Display for Att<'a, Load> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "movl {}, {}", Att(&self.0.memory), Att(&self.0.register))
    }
}

impl<'a> fmt::Display for Att<'a, Store> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "movl {}, {}", Att(&self.0.value), Att(&self.0.memory))
    }
}

impl<'a> fmt::Display for Att<'a, Mov> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "movl {}, {}", Att(&self.0.value), Att(&self.0.register))
    }
}

impl<'a> fmt::Display for Att<'a, Add> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "addl {}, {}", Att(&self.0.value), Att(&self.0.register))
    }
}

impl<'a> fmt::Display for Att<'a, Push> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pushl {}", Att(&self.0.value))
    }
}

impl<'a> fmt::Display for Att<'a, Neg> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "negl {}", Att(&self.0.register))
    }
}

impl<'a> fmt::Display for Att<'a, Call> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "call {}", self.0.label)
    }
}

impl<'a> fmt::Display for Att<'a, Ret> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ret")
    }
}


impl<'a> fmt::Display for Att<'a, Instruction> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;
        match *self.0 {
            Load(ref instr) => write!(f, "{}", Att(instr)),
            Store(ref instr) => write!(f, "{}", Att(instr)),
            Mov(ref instr) => write!(f, "{}", Att(instr)),
            Add(ref instr) => write!(f, "{}", Att(instr)),
            Push(ref instr) => write!(f, "{}", Att(instr)),
            Neg(ref instr) => write!(f, "{}", Att(instr)),
            Call(ref instr) => write!(f, "{}", Att(instr)),
            Ret(ref instr) => write!(f, "{}", Att(instr)),
        }
    }
}

impl<'a> fmt::Display for Att<'a, Program> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
.globl main
main:
    pushl %ebp
    movl %esp, %ebp
    subl ${}, %esp
", self.0.stack_len * 4)?;

        for instruction in &self.0.instructions {
            writeln!(f, "    {}", Att(instruction))?;
        }

        write!(f, "
    movl $0, %eax
    leave
    ret
")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use Memory;
    use Register;
    use Immediate;
    use ScaleFactor;
    use Displacement;
    use ia_32::Load;

    #[test]
    fn fmt_memory() {
        let memory = Memory {
            base: Register::EDI,
            index: None,
            displacement: Displacement(0),
        };
        let memory = format_att!("{}", memory);
        assert_eq!(memory, "0(%edi)");

        let memory = Memory {
            base: Register::ESP,
            index: Some((Register::EBP, ScaleFactor::one())),
            displacement: Displacement(126),
        };
        let memory = format_att!("{}", memory);
        assert_eq!(memory, "126(%esp, %ebp, 1)");
    }

    #[test]
    fn fmt_load() {
        // [ecx * 8 + eax + 64]
        let memory = Memory {
            base: Register::EAX,
            index: Some((Register::ECX, ScaleFactor::eight())),
            displacement: Displacement(64)
        };
        let load = Load {
            memory,
            register: Register::EDX,
        };
        let load = format_att!("{}", load);
        assert_eq!(load, "mov 64(%eax, %ecx, 8), %edx");
    }
}
