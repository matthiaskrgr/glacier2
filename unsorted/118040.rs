#![feature(associated_const_equality)]

fn f(_: impl Trait<K = 0i32>) {}

trait Trait: Super {}
trait Super { const K: i32; }

fn main() {}
