use bb;
use val::Val;

pub enum Term {
    Goto(bb::Label),
    SwitchInt {
        cond: Val,
        goto: Vec<(i32, bb::Label)>
    },
    Return,
}
