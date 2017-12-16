use vm::FuncData;
use vm::Block;
use vm::BlockData;
use vm::VarEnv;
use std::collections::HashMap;
use vm::Var;
use vm::Inst;
use vm::InstData;
use vm::Lval;
use vm::Rval;

// SSA val
pub type Val = Var;

pub struct Builder<'func> {
    curr_def: HashMap<Var, HashMap<Block, Val>>,
    sealed: HashMap<Block, BlockData>,
    incomplete_phis: HashMap<Block, HashMap<Var, Block>>,
    vals: VarEnv,
    func: &'func FuncData,
}

impl<'func> Builder<'func> {
    pub fn new(func: &FuncData) -> Builder {
        Builder {
            curr_def: HashMap::new(),
            sealed: HashMap::new(),
            incomplete_phis: HashMap::new(),
            vals: VarEnv::new(),
            func: func,
        }
    }

    pub fn write(&mut self, var: Var, block: Block, value: Var) {
        if !self.curr_def.contains_key(&var) {
            self.curr_def.insert(var, HashMap::new());
        }
        let inner = self.curr_def.get_mut(&var).unwrap();
        inner.insert(block, value);
    }

    pub fn read(&self, var: &Var, block: &Block) -> Var {
        if self.curr_def[var].contains_key(block) {
            return self.curr_def[var][block]
        }
        return self.read_recursive(var, block)
    }

    pub fn read_lval(&self, lval: &Lval, block: &Block) -> Lval {
        match *lval {
            Lval::Var(ref var) => Lval::Var(self.read(var, block)),
            _ => lval.clone(),
        }
    }

    pub fn read_rval(&self, rval: &Rval, block: &Block) -> Rval {
        match *rval {
            Rval::Lval(ref lval) => Rval::Lval(self.read_lval(lval, block)),
            _ => rval.clone(),
        }
    }

    pub fn read_recursive(&self, var: &Var, block: &Block) -> Var {
        if !self.sealed.contains_key(block) {
            
        }
        unimplemented!()
    }

    pub fn visit_block(&mut self, block: &Block) {
        let mut body = Vec::new();
        for inst in self.func.block(block).body.iter() {
            let new = self.convert_inst(inst, block);
            body.push(new);
        }
    }

    pub fn convert_inst(&mut self, inst: &Inst, block: &Block) -> Inst {
        let data = match inst.data {
            InstData::Unary { ref arg, opcode } => {
                let arg = self.read_rval(arg, block);
                InstData::Unary { opcode, arg }
            }
            InstData::Binary { opcode, ref left, ref right } => {
                let left = self.read_rval(left, block);
                let right = self.read_rval(right, block);
                InstData::Binary { opcode, left, right }
            }
            InstData::CallIndirect { ref target, ref args } => {
                let target = self.read_lval(target, block);
                let args = args.iter().map(|arg| self.read_rval(arg, block)).collect();
                InstData::CallIndirect { target, args }
            }
            InstData::Call { ref func, ref args } => {
                let func = func.to_owned();
                let args = args.iter().map(|arg| self.read_rval(arg, block)).collect();
                InstData::Call { func, args }
            }
            InstData::ShiftLeftThenOr { ref arg, shift, or } => {
                let arg = self.read_rval(arg, block);
                InstData::ShiftLeftThenOr { arg, shift, or }
            }
            InstData::MovFuncLabel { ref func } => {
                InstData::MovFuncLabel { func: func.clone() }
            }
            InstData::Phi { .. } => panic!("Encountered phi function while converting to ssa form??"),
        };
        unimplemented!()
    }

    pub fn build(&mut self) {
        let root = &self.func.root().name;
        self.visit_block(root);
    }
}

pub fn convert_to_ssa2(func: &mut FuncData, vars: &mut VarEnv) -> FuncData {
    let mut builder = Builder::new(func);
    builder.build();
    unimplemented!()
}
