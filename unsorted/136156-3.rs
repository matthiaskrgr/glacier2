#![feature(generic_const_items)]

const _CONSUME<T>: fn(T) = |_| (); // or the below
//const _CONSUME<T>: fn(T) = consume;
//fn consume<T>(_: T) {}

fn main() {}
