//!
//! Parser for pyil
//!
//! Example of Python program and its il:
//!
//! Python P2
//!
//!     x = 2
//!     y = True
//!     z = [1, 2, 3]
//!     w = {1:2, 3:4}
//!
//!     print x + x
//!     print -x
//!     print not y
//!     print y and x
//!     print 0 or x
//!     print z == z
//!     print z != z
//!     print z is z
//!     print z is w
//!     print 1 is 1
//!     print z if y else w
//!
//! Python IL
//!
//!     func main() -> int {
//!     bb0:
//!         v0: int = copyi32 2
//!         v1: pyobj = call inject_int(v0)
//!
//!         v2: int = copyi32 1
//!         v3: pyobj = call inject_bool(v2)
//!
//!         v4: *big_pyobj = call create_list(3)
//!         v5: pyobj = call inject_big(v4)
//!
//!         v6: int = copyi32 0
//!         v7: pyobj = call inject_int(v6)
//!
//!         v8: int = copyi32 1
//!         v9: pyobj = call inject_int(v8)
//!
//!         v10: int = copyi32 2
//!         v11: pyobj = call inject_int(v10)
//!
//!         v12: int = copyi32 1
//!         v13: pyobj = call inject_int(v12)
//!         v14: () = call set_subscript(v5, v7, v13)
//!
//!         v15: int = copyi32 2
//!         v16: pyobj = call inject_int(v15)
//!         v17: () = call set_subscript(v5, v9, v16)
//!
//!         v18: int = copyi32 3
//!         v19: pyobj = call inject_int(v18)
//!         v20: () = call set_subscript(v5, v11, v19)
//!
//!         v21: *big_pyobj = call create_dict()
//!         v22: pyobj = call inject_big(v21)
//!
//!         v23: int = copyi32 1
//!         v24: pyobj = call inject_int(v23)
//!         v25: int = copyi32 2
//!         v26: pyobj = call inject_int(v25)
//!         v27: () = call set_subscript(v22, v24, v26)
//!
//!         v28: int = copyi32 3
//!         v29: pyobj = call inject_int(v28)
//!         v30: int = copyi32 4
//!         v31: pyobj = call inject_int(v30)
//!         v32: () = call set_subscript(v22, v29, v31)
//!
//!         v33: int = call tag(v1)
//!         // this switch becomes
//!         //
//!         // cmpl 0, v33
//!         // je bb1
//!         // cmpl 1, v33
//!         // je bb1
//!         // cmpl 3, v33
//!         // je bb2
//!         // jmp unexpected_int_in_switch
//!         //
//!         // note that no more virtual
//!         // locations are needed as long as
//!         // the conditions are constant ints
//!         switch v33 [0 -> bb1, 1 -> bb1, 3 -> bb2]
//!         
//!     bb1:
//!         // first arg is int or bool,
//!         // check if second arg to + is int or bool
//!         v34: int = call tag(v1)
//!         switch v34 [0 -> bb3, 1 -> bb3]
//!
//!     bb2:
//!         // first arg is big,
//!         // check if second arg is big
//!         v35: int = call tag(v1)
//!         switch v35 [3 -> bb4]
//!
//!     bb3:
//!         // both arguments to + are int or bool
//!         // TODO There should really be another branch
//!         // to check if bool or int, rather than just shifting.
//!         // Oh well, sue me lol.
//!         v36: int = shr 2, v1
//!         v37: int = shr 2, v1
//!         v38: int = add v36, v37
//!         v39: pyobj = call inject_int(v38)
//!         goto bb5
//!
//!     bb4:
//!         // both arguments to + are big
//!         v40: *big_pyobj = call project_big(v1)
//!         v41: *big_pyobj = call project_big(v1)
//!         v42: *big_pyobj = call add(v40, v41)
//!         v43: pyobj = call inject_big(v42)
//!         goto bb5
//!
//!     bb5:
//!         // phi gets removed at print_asm
//!         // phase, mov v39, v44 gets appended to
//!         // bb3 and mov v43, v44 gets appended to
//!         // bb4
//!         v44: pyobj = phi(v39, v43)
//!         v45: () = call print_any(v44)
//!
//!         v46: int = shr 2, v1
//!         v47: int = neg v46
//!         v48: pyobj = call inject_int(v47)
//!         v49: () = call print(v48)
//!
//!         v50: int = call is_true(v2)
//!         v51: int = not v50
//!         v52: () = call print(v51)
//!     }
//!
//!
//!     print 1 if True else [1]
//!
//!     // should become
//!
//!     bb0:
//!         v0: int = copyi32 1
//!         v1: pyobj = call inject_bool(v0)
//!         v2: int = call is_true(v1)
//!         switch v2 [0 -> bb1, 1 -> bb2]
//!
//!     bb1:
//!         
//!         goto bb3
//!
//!     bb2:
//!         
//!         goto bb2
//!
//!     bb3:
//!         v3: pyobj = phi(
//!         
//!
#![feature(conservative_impl_trait, slice_concat_ext)]
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
