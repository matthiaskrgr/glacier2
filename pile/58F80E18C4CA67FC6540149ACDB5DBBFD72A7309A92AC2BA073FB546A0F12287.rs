// Check that we can specialize on a concrete iterator type. This requires us
// to consider which parameters in the parent impl are constrained.

// check-pass

#![feature(min_specialization)]

trait SpecFromIter<T> {
    fn f(&self);
}

impl<'c, T: 'a, I: 'a> SpecFromIter<T> for I {
    default type U = str;
    //~^ ERROR the trait bound `str: Clone` is not satisfied
}

impl<'a, T> SpecFromIter<T> for std::slice::Iter<'a, T> {
    fn f(&self) {}
}

fn main() {}
