// ignore-test (This is currently broken)

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const_pat)]

use std::stmt::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'static i32>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<T> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn main() {
    const {
        require_unsafe();
        //~^ ERROR [E0133]
    }
}

fn main() {
    match_invariant_ref();
}
