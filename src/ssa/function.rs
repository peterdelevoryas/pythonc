use ssa::ProgramBuilder;

impl_ref!(Function, "f");
pub type FunctionGen = Gen;
pub type FunctionMap<T> = Slab<T>;

pub struct FunctionData {
    is_main: bool,
}

pub struct Builder<'a> {
    program: &'a mut ProgramBuilder,
    is_main: bool,
}

impl<'a> Builder<'a> {
    pub fn new(program: &'a mut ProgramBuilder) -> Self {
        Builder { program, is_main: false }
    }

    pub fn is_main(&mut self, is_main: bool) {
        self.is_main = is_main;
    }

    pub fn build(self) -> FunctionData {
        FunctionData {
            is_main: self.is_main,
        }
    }
}
