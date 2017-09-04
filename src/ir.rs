//!
//! Below is a running example of how I think p0 should
//! translate to the IR in this module:
//!
//!     x = 1 + 2
//!     y = 3 + 4
//!     print x + y
//!     z = -x + 1 + y
//!     print z
//!
//!
//!     
//!
//!
//! In general, here are the mappings between `ast::Statement`s and `ir::Statement`s
//!
//!     Expression::DecimalI32(value) -> {
//!         tmpN = value
//!     }
//!
//!     Assign(Name, Expr) -> {
//!         tmpN = ir(expr)
//!     }
//!

use ast;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    stmts: Vec<Stmt>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Tmp {
    index: usize
}

/// Tmp(index) -> index of Tmp in stack
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Int(i32),
    Ref(Tmp),
}

///
///     tmp8 = tmp7     ; Copy(Tmp)
///     tmp0 = 1        ; Copy(Constant)
///     tmp1 = -1       ; UnaryNeg(Constant)
///     tmp2 = -tmp1    ; UnaryNeg(Tmp)
///     tmp3 = 1 + 2    ; Add(Constant, Constant)
///     tmp4 = 1 + tmp3 ; Add(Constant, Tmp)
///     tmp5 = tmp4 + 1     ; Add(Tmp, Constant)
///     tmp6 = tmp4 + tmp5  ; Add(Tmp, Tmp)
///     tmp7 = input()  ; Input
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Copy(Val),
    UnaryNeg(Val),
    Add(Val, Val),
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Print(Val),
    Def(Tmp, Expr),
}

#[derive(Debug)]
pub struct Builder {
    stack: Vec<Stmt>,
    names: HashMap<ast::Name, Tmp>,
    tmp: TmpAllocator,
}

#[derive(Debug)]
pub struct TmpAllocator {
    next: usize,
}

impl TmpAllocator {
    pub fn new() -> TmpAllocator {
        TmpAllocator {
            next: 0
        }
    }

    pub fn alloc(&mut self) -> Option<Tmp> {
        let tmp = Tmp {
            index: self.next
        };
        self.next = match self.next.checked_add(1) {
            Some(sum) => sum,
            None => return None,
        };
        Some(tmp)
    }

    
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            stack: vec![],
            names: HashMap::new(),
            tmp: TmpAllocator::new(),
        }
    }

    pub fn flatten_expression(&mut self, expression: &ast::Expression) -> Val {
        match *expression {
            ast::Expression::DecimalI32(ast::DecimalI32(i)) => {
                Val::Int(i)
            }
            ast::Expression::Name(ref name) => {
                let tmp = match self.names.get(name) {
                    Some(&tmp) => tmp,
                    None => panic!("reference to undefined name {:?}", name),
                };
                Val::Ref(tmp)
            }
            ast::Expression::Input(_) => {
                let tmp = self.def(Expr::Input);
                Val::Ref(tmp)
            }
            ast::Expression::UnaryNeg(ref expr) => {
                let val = self.flatten_expression(expr);
                let tmp = self.def(Expr::UnaryNeg(val));
                Val::Ref(tmp)
            }
            ast::Expression::Add(ref left, ref right) => {
                let left = self.flatten_expression(left);
                let right = self.flatten_expression(right);
                let tmp = self.def(Expr::Add(left, right));
                Val::Ref(tmp)
            }
        }
    }

    pub fn flatten_statement(&mut self, statement: &ast::Statement) {
        match *statement {
            ast::Statement::Print(ref expression) => {
                let val = self.flatten_expression(expression);
                self.print(val);
            }
            ast::Statement::Assign(ref name, ref expression) => {
                let val = self.flatten_expression(expression);
                let copy = Expr::Copy(val);
                let tmp = self.def(copy);
                self.names.insert(name.clone(), tmp);
            }
            ast::Statement::Expression(ref expression) => {
                let _ = self.flatten_expression(expression);
            }
            ast::Statement::Newline => {}
        }
    }

    pub fn push(&mut self, s: Stmt) {
        self.stack.push(s);
    }

    pub fn def(&mut self, expr: Expr) -> Tmp {
        let tmp = self.tmp.alloc().expect("tmp allocator oom");
        let def = Stmt::Def(tmp, expr);
        self.push(def);
        tmp
    }

    pub fn print(&mut self, val: Val) {
        let print = Stmt::Print(val);
        self.push(print);
    }

    pub fn stack(&self) -> &[Stmt] {
        &self.stack
    }
}

#[cfg(test)]
mod test {
}
