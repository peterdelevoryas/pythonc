/*
mod program;
pub use self::program::Program;
*/

use std::collections::HashMap;

mod value;
pub use self::value::Value;
pub use self::value::ValueGen;
pub use self::value::ValueMap;

mod program;
pub use self::program::Program;
pub use self::program::Builder as ProgramBuilder;

mod function;
pub use self::function::Function;
pub use self::function::FunctionData;
pub use self::function::FunctionMap;
pub use self::function::Builder as FunctionBuilder;

mod expr;
pub use self::expr::Expr;
pub use self::expr::Unary;
pub use self::expr::Binary;
pub use self::expr::CallTarget;
pub use self::expr::Phi;
pub use self::expr::Builder as ExprBuilder;

mod branch;
pub use self::branch::Branch;
pub use self::branch::Jmp;
pub use self::branch::Jnz;
pub use self::branch::Ret;

mod block;
pub use self::block::Block;
pub use self::block::BlockData;
pub use self::block::BlockMap;

mod liveness;
pub use self::liveness::LiveSets;
pub use self::liveness::LiveSet;
pub use self::liveness::LiveVal;

pub mod solver;
use self::solver::Graph;
use self::solver::Coloring;

pub fn allocate_registers(function: &mut FunctionData) -> Coloring {
    let mut coloring = Coloring {
        next_spill: 0,
        colors: HashMap::new(),
    };
    loop {
        use self::solver::DSaturResult::*;
        let mut g = Graph::build(function, &coloring.colors);
        match g.run_dsatur(&mut coloring) {
            Success => {
                break;
            }
            Spill(value) => {
                let spill = ::stack::Slot::Spill { index: coloring.next_spill };
                coloring.colors.insert(::ssa::solver::Node::Value(value), ::ssa::solver::Color::Stack(spill));
                coloring.next_spill += 1;
                //replace_stack_to_stack_ops()
            }
        }
    }
    coloring
}
