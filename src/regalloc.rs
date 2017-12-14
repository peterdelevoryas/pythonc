use vasm;
use graph;
type Vars = ::explicate::var::Slab<::explicate::var::Data>;

/// Outputs completely register-allocated assembly or panics
pub fn regalloc(mut function: vasm::Function, vars: &mut Vars) -> vasm::Function {
    loop {
        use graph::DSaturResult::*;
        let mut g = graph::Graph::build(&function);
        match g.run_dsatur() {
            Success => {
                function = g.assign_homes(function);
                return function
            }
            Spill(u) => {
                println!("spilling {}", u);
                function.spill(u);
                function = function.replace_stack_to_stack_ops(vars);
            }
        }
    }
}
