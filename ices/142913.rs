#![feature(generic_const_parameter_types)]
struct Variant;

fn foo<'a, const N: &'a Variant = {}>() {}

fn main() {}
