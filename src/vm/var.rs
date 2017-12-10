use std::fmt;
use explicate::var;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Var {
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Data {
    Temp,
    User { name: String },
}

impl Var {
    pub fn from(var: var::Var) -> Var {
        Var { index: var.inner() }
    }
}

impl Data {
    pub fn temp() -> Self {
        Data::Temp
    }

    pub fn user(name: String) -> Self {
        Data::User { name }
    }
}

pub struct Env {
    next: usize,
    pub map: HashMap<Var, Data>,
}

impl Env {
    pub fn from(var_data: &var::Slab<var::Data>) -> Env {
        let next = var_data
            .iter()
            .map(|(v, _)| v.inner())
            .max()
            .map(|max| max + 1)
            .unwrap_or(0);
        let mut map = HashMap::new();
        for (var, data) in var_data {
            let var = Var { index: var.inner() };
            let data = match *data {
                var::Data::User { ref source_name } => Data::user(source_name.clone()),
                var::Data::Temp => Data::temp(),
            };
            map.insert(var, data);
        }
        Env { next, map }
    }

    pub fn new_temp(&mut self) -> Var {
        let temp = Data::Temp;
        let var = Var { index: self.next };
        self.next += 1;
        self.map.insert(var, temp);
        var
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "_{}", self.index)
    }
}
