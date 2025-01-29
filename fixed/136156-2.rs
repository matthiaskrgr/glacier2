#![feature(generic_const_items)]

const _IDENTITY<T>: fn(T) -> T = identity;
fn identity<T>(x: T) -> T { x }

fn main() {}
