//!
//! Parser for pyil
//!
//! Example of Python program and its il:
//!
//! Python P2
//!
//!     0   x = 1 + 2
//!     1   y = 3 + input()
//!     2   z = x + -input() + y
//!     3   if x == 3:
//!     4       z = -z
//!     5   else:
//!     6       y = -2
//!     7   print x
//!     8   print y
//!     9   print z
//! 
//!
//! Python IL
//!
//!     func main() -> i32 {
//!         val v0: i32; // x, line 0
//!         val v1: i32;
//!         val v2: i32; // y, line 1
//!         val v3: i32;
//!         val v4: i32;
//!         val v5: i32;
//!         val v6: i32; // z, line 2
//!         val v7: i32; // z, line 4
//!         val v8: i32; // y, line 6
//!         val v11: (); // _, line 7
//!         val v12: (); // _, line 8
//!         val v13: (); // _, line 9
//!         val v14: i32; // ret val
//! 
//!         block bb0() {
//!             v0 = add(1, 2);
//!             v1 = call(input, []);
//!             v2 = add(3, v2);
//!             v3 = call(input, []);
//!             v4 = neg(v3);
//!             v5 = add(v0, v4);
//!             v6 = add(v5, v2);
//!             switch v0 {
//!                 3 -> bb1;
//!                 _ -> bb2;
//!             }
//!         }
//! 
//!         block bb1() {
//!             v7 = neg(v6);
//!             goto bb3(v2, v7);
//!         }
//! 
//!         block bb2() {
//!             v8 = -2;
//!             goto bb3(v8, v6);
//!         }
//!         
//!         // v9 -> y, line 8
//!         // v10 -> z, line 9
//!         block bb3(v9: i32, v10: i32) {
//!             v11 = call(print, [v0]);
//!             v12 = call(print, [v9]);
//!             v13 = call(print, [v10]);
//!             goto bb4();
//!         }
//! 
//!         block bb4() {
//!             v14 = 0;
//!             ret;
//!         }
//!     }
//!
//!
#![feature(conservative_impl_trait)]
extern crate lalrpop_util;
extern crate regex;
extern crate slab as slab_crate;
extern crate python_ast as ast;

#[macro_use]
pub mod slab;
pub mod syntax;
pub mod func;
pub mod term;
pub mod bb;
pub mod inst;
pub mod val;
pub mod ty;
