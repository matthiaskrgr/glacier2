#![feature(generic_const_exprs)]
use std::ops::Deref;

trait Trait {
    const CONST: usize = 0;
}
impl Trait for () {}

trait DynTrait {
    fn f(&self) {}
}

impl<T> DynTrait for T
where
    T: Trait,
    [(); T::CONST]:,
{
}

struct Wrap;

impl Deref for Wrap {
    type Target = ();
    fn deref(&self) -> &() {
        &()
    }
}

fn main() {
    Wrap.f();
}
