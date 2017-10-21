use bb;
use val::Val;

pub enum Term {
    Goto(bb::BasicBlock),
    SwitchInt {
        cond: Val,
        goto: Vec<(i32, bb::BasicBlock)>
    },
    Return,
}
