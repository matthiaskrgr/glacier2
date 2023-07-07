#![feature(associated_type_bounds)]
#![allow(Point { x: 42, y: 24 })]

pub struct Reverse<T>(T);
trait T {}

impl const S {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}
//~^ ERROR inherent impls cannot be `const`

impl const ST6 {}
//~^ ERROR inherent impls cannot be `const`

fn main() {}
