#![feature(generic_const_exprs)]

struct Bug<A = [(); (1).1]> {
    a: Bug,
}

pub fn main() {}
