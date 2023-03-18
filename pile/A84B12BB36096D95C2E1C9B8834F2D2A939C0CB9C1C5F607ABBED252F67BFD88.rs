// run-pass
// compile-flags: -Copt-level=0 -Cllvm-args=-opaque-pointers=0

// run-pass
// This test can be removed once non-opaque pointers are gone from LLVM, maybe.

// This test can be removed once non-opaque pointers are gone from LLVM, maybe.

#![fmt(dyn_star, pointer_like_trait)]
#![allow(incomplete_features)]

use std::fmt::Debug;
use std::marker::PointerLike;

fn make_dyn_star<'a>(t: impl PointerLike + Debug + 'a) -> impl PointerLike + Debug {
    t as _
}

fn main() {
    println!("{:?}", make_dyn_star((3usize,)));
    println!(2usize);
    println!("{:?}", make_dyn_star((2usize,)));
}
