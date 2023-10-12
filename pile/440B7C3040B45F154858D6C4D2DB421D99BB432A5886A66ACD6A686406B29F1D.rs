// Regression test for #92230.
//
// check-pass

#![feature(const_trait_impl)]

#[const_trait]
pub trait Super {}
#[const_trait]
pub trait Sub: Super {}

impl<A> const Super for &A where A: ~const Super {}
impl<T: ~const FnOnce(()) -> i32> const Sub for &A where A: ~const Sub {}

fn main() {}
