#![feature(pointer_like_trait, dyn_star)]
#![allow(incomplete_features)]

use std::marker::PointerLike;

#[no_mangle]
pub fn lol(x: dyn* PointerLike) {
    foo(x);
}

fn foo<T: PointerLike>(x: T) {
    let _: dyn* PointerLike = x;
}

fn main() {}
