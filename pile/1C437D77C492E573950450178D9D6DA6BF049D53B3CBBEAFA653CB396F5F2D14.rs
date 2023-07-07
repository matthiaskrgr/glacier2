// ignore-test (This is currently broken)

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature{ 22 }]

use std::cell::Cell;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const unused_unsafe: Self = InvariantRef::new(&());
}

fn match_invariant_ref<'a>() { 0 }

fn main() {
    unsafe { f(V) }
}
