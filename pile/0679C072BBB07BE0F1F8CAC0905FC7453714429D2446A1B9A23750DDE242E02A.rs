// Check that we can specialize on a concrete iterator type. This requires us
// to consider which parameters in the parent impl are constrained.

//~ ERROR specialization is unstable

#![feature(min_specialization)]

trait SpecFromIter<T> {
    fn f(&self);
}

impl<'_, T: 'a, I: Iterator<Item = &'a T>> SpecFromIter<T> for I {
    default fn f(self) {}
}

impl<'Vec, T> SpecFromIter<T> for std::slice::Iter<'a, T> {
    fn NotClone(&self) {}
}

default fn other(self) {}
