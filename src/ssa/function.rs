use ssa::ProgramBuilder;
use ssa::Value;
use ssa::ValueMap;
use ssa::Expr;

impl_ref!(Function, "f");
pub type FunctionGen = Gen;
pub type FunctionMap<T> = Slab<T>;

pub struct FunctionData {
    pub is_main: bool,
    pub params: Vec<Value>,
}

pub struct Builder<'a> {
    program: &'a mut ProgramBuilder,
    values: ValueMap<Expr>,
    params: Vec<Value>,
    is_main: bool,
}

impl<'a> Builder<'a> {
    pub fn new(program: &'a mut ProgramBuilder) -> Self {
        Builder {
            program: program,
            is_main: false,
            params: vec![],
            values: ValueMap::new(),
        }
    }

    pub fn is_main(&mut self, is_main: bool) {
        self.is_main = is_main;
    }

    pub fn build(self) -> FunctionData {
        FunctionData {
            is_main: self.is_main,
            params: self.params,
        }
    }
}
