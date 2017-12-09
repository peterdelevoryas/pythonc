use std::fmt;
use explicate::var;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var {
    inner: Inner,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner {
    Temp,
    User { name: String },
}

impl Var {
    pub fn temp(index: usize) -> Self {
        Var {
            inner: Inner::Temp,
            index: index,
        }
    }

    pub fn user(index: usize, name: String) -> Self {
        Var {
            inner: Inner::User { name },
            index: index,
        }
    }
}

pub struct Env {
    next: usize,
}

impl Env {
    pub fn from(var_data: &var::Slab<var::Data>) -> Env {
        let next = var_data
            .iter()
            .map(|(v, _)| v.inner())
            .max()
            .map(|max| max + 1)
            .unwrap_or(0);
        Env { next }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            Inner::Temp => write!(f, "_{}", self.index),
            Inner::User { ref name } => write!(f, "{}.{}", name, self.index),
        }
    }
}
