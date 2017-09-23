use std::fmt;
use ia_32::*;
use register::Register;
use memory::Memory;
use immediate::Immediate;

/// Wrapper struct used to implement
/// custom AT&T syntax formatting
pub struct Att<T>(T);

impl fmt::Display for Att<Load> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mov {}, %{}", Att(self.0.source), Att(self.0.register))
    }
}

impl fmt::Display for Att<Memory> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.index {
            Some((index, scale_factor)) => {
                write!(f, "{displacement}(%{base}, %{index}, {scale_factor})",
                    displacement=self.0.displacement.value,
                    base=self.0.base.as_str(),
                    index=index.as_str(),
                    scale_factor=scale_factor.value(),
                )
                    
            }
            None => {
                write!(f, "{displacement}(%{base})",
                    displacement=self.0.displacement.value,
                    base=self.0.base.as_str(),
                )
            }
        }
    }
}

impl fmt::Display for Att<Register> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
