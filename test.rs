use std::fmt;

pub struct Func {
    name: String,
    code: Vec<String>,
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}() {{", self.name)?;
        for line in &self.code {
            writeln!(f, "{:4}", line)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

fn main() {
    println!("{}", "_".repeat(4));
}
