extern crate python_ir as ir;

pub mod instruction;
pub use instruction::Instruction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
