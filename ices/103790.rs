#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct S<const S: (), const S: S = {S}>;

fn main() {}
