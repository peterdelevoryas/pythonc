use std::fmt;
use ia_32::*;
use register::Register;
use memory::Memory;
use memory::ScaleFactor;
use memory::Displacement;
use immediate::Immediate;

/// Not quite as great as write!, because
/// we have to match on expr's instead of
/// tt's in order to wrap arguments in
/// Att<T>. For example, can't do keyword
/// arguments. It also requires at least one
/// argument to format, because otherwise
/// it would be better to just use write!().
#[macro_export]
macro_rules! write_att {
    ($dst:expr, $fmt:expr, $($arg:expr),+) => ($dst.write_fmt(format_args!($fmt, $($crate::att::Att($arg)),+)))
}

#[macro_export]
macro_rules! format_att {
    ($fmt:expr, $($arg:expr),+) => (::std::fmt::format(format_args!($fmt, $($crate::att::Att($arg)),+)))
}

/// Wrapper struct used to implement
/// custom AT&T syntax formatting
pub struct Att<T>(T);

impl fmt::Display for Att<Load> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_att!(f, "mov {}, {}", self.0.memory, self.0.register)
    }
}

impl fmt::Display for Att<Displacement> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Instead of writing $2, write 2 by itself
        // Also, apparently the Rust parser
        // can't handle self.0.0 without parens
        write!(f, "{}", (self.0).0)
    }
}

impl fmt::Display for Att<Immediate> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", (self.0).0)
    }
}

impl fmt::Display for Att<Register> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.0.as_str())
    }
}

impl fmt::Display for Att<ScaleFactor> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.value())
    }
}

impl fmt::Display for Att<Memory> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.index {
            Some((index, scale_factor)) => {
                write_att!(f, "{}({}, {}, {})", self.0.displacement, self.0.base, index, scale_factor)
                    
            }
            None => {
                write_att!(f, "{}({})", self.0.displacement, self.0.base)
            }
        }
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
