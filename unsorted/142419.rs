#![feature(lazy_type_alias)]
type A = ();
type B = (A, A, A, A);
type C = (B, B, B, B, B, B, B);
type D = (C, C, C, C);
fn foo(x: D) {}
fn main() {}
