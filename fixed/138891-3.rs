#![feature(trait_alias)]
trait A = Fn() -> Self;
fn c(a: dyn A) {}

pub fn main() {}
