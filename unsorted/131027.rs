#![feature(trait_upcasting)]
trait Supertrait<'a, 'b> {}

trait Subtrait<'a, 'b>: Supertrait<'a, 'b> {}

pub fn ok(x: &dyn for<'a, 'b> Subtrait<'a, 'b>) -> &dyn for<'a> Supertrait<'a, 'a> {
    x
}

pub fn main() {}
