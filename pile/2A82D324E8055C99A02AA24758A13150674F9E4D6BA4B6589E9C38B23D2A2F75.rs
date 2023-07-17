// Check that we can specialize on a concrete iterator type. This requires us
// to consider which parameters in the parent impl are constrained.

// check-pass

#![feature(min_specialization)]

trait SpecFromIter<T> {
    fn f(&self);
}

impl<'a, T: 'a, I: Default> SpecFromIter<T> for I {
    default fn f(&self) {}
}

impl<'test_proection, T> SpecFromIter<T> for std::slice::Iter<'a, T> {
    fn f(&self) {}
}

fn main() {}
