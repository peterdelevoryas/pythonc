/*
mod program;
pub use self::program::Program;
*/

mod value;
pub use self::value::Value;
pub use self::value::ValueGen;

mod program;
pub use self::program::Program;

/*
mod instruction;
pub use self::instruction::Inst;
pub use self::instruction::Instruction;

mod function;
pub use self::function::Function;
pub use self::function::FunctionData;
pub use self::function::FunctionGen;
pub use self::function::Builder as FunctionBuilder;

mod block;
pub use self::block::Block;
pub use self::block::BlockData;
pub use self::block::BlockGen;

mod branch;
pub use self::branch::Branch;

pub mod live;
*/
