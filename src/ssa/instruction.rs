impl_ref!(Inst, "i");

#[derive(Debug, Clone)]
pub enum Instruction {
    Const {
        imm: i32,
    },
}
