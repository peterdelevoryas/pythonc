pub mod module;
pub use self::module::Module;

pub mod var;
pub use self::var::Var;
pub use self::var::Data as VarData;
pub use self::var::Env as VarEnv;

pub mod func;
pub use self::func::Func;
pub use self::func::Data as FuncData;

pub mod reg;
pub use self::reg::Reg;
pub use self::reg::Reg::*;

pub mod inst;
pub use self::inst::Inst;
pub use self::inst::Data as InstData;
pub use self::inst::Imm;
pub use self::inst::Lval;
pub use self::inst::Rval;
pub use self::inst::Unary;
pub use self::inst::Binary;

pub mod stack;
pub use self::stack::Slot as StackSlot;

pub mod term;
pub use self::term::Term;

pub mod block;
pub use self::block::Block;
pub use self::block::Data as BlockData;

pub mod liveness;
pub use self::liveness::Lvals;
pub use self::liveness::Liveness;

pub mod util;
pub use self::util::fmt_indented;

pub mod visit;
pub use self::visit::Visit;

pub mod interference;
pub use self::interference::Graph as InterferenceGraph;

pub mod ssa;
pub use self::ssa::convert_to_ssa;

pub mod ssa2;
pub use self::ssa2::convert_to_ssa2;
