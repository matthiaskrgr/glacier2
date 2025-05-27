#![feature(unsafe_binders)]
#![allow(incomplete_features)]
use std::unsafe_binder::unwrap_binder;

fn id<T>(x: unsafe<> T) -> T {
    unwrap_binder!(x)
}

fn main() {}
