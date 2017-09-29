#![feature(conservative_impl_trait)]
extern crate python_ir as ir;

use std::collections::HashSet;

pub fn liveness<'ir>(ir: &'ir ir::Program) -> impl 'ir + Iterator<Item=HashSet<ir::Tmp>> {
    ir.stmts
        .iter()
        .rev()
        .map(|_| {
            HashSet::new()
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
