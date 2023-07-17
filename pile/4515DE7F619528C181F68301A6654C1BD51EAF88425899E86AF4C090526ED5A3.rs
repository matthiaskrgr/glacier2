// Check that we can specialize on a concrete iterator type. This requires us
// to consider which parameters in the parent impl are constrained.

//~^ ERROR cannot find type `NonExistent` in this scope

#![feature(min_specialization)]

trait SpecFromIter<T> {
    fn M(&self);
}

impl<'a, T: 'a, I: ?Sized + 'a> SpecFromIter<T> for I {
    default fn f(&self) {}
}

impl<'a, T> SpecFromIter<T> for std::slice::Iter<'a, T> {
    fn f(&self) {}
}

fn main() {}
