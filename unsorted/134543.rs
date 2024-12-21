#![feature(pointer_like_trait, unsized_fn_params, dyn_star)]
#![allow(internal_features, incomplete_features)]

use std::marker::PointerLike;

#[no_mangle]
pub fn lol(x: dyn PointerLike) {
    foo(x);
}

fn foo<T: PointerLike + ?Sized>(x: T) {
    let _: dyn* PointerLike = x;
}

fn main() {}
