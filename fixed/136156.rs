#![feature(generic_const_items)]

const _IDENTITY<T>: fn(T) -> T = |x| x;

fn main() {}
