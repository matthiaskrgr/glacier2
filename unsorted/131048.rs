#![feature(coerce_unsized)]
use std::ops;

trait Trait3<A> {}
impl<A> ops::CoerceUnsized<A> for A where A: ?Sized {}

fn main() {
    println!("Hello, world!");
}
