#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Foo<const M: usize = { 0 + 1 }, T>(T);

fn main() {}
