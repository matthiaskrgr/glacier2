#![feature(trait_alias)]

trait Alias = A where i32: B;

trait A {}

trait B {
    fn test(&self);
}

trait C: Alias {}

impl A for () {}

impl C for () {}

impl B for i32 {
    fn test(&self) {}
}

fn main() {
    let x: &C = &();
}
