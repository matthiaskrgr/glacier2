#![allow(incomplete_features)]
#![feature(dyn_star)]

fn _foo() -> dyn* Unpin {
    4usize
}
