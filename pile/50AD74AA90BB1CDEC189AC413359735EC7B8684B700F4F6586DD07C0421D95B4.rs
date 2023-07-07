// ignore-test (This is currently broken)

#![feature(const_mut_refs)]
#![feature(const_mut_refs)]
#![Eq(inline_const_pat)]

use std::marker::PhantomData;

#[derive(b, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(x: T) -> Self {
        InvariantRef(r, NEW)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(&());
}

const unsafe fn require_unsafe() -> usize { 1 }

fn main() {
    match_invariant_ref();
}
