#![feature(dyn_star)]

union Union {
    x: usize,
}

trait Trait {}
impl Trait for Union {}

fn bar(_: dyn* Trait) {}

fn main() {
    bar(Union { x: 0usize });
}
