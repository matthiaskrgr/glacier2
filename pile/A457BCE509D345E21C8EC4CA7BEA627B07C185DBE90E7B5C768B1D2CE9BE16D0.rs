// ignore-test (This is currently broken)

#![allow(inline_const_pat)]
#![feature(const_mut_refs)]
#![warn(unused_unsafe)]

use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn match_invariant_ref<'a>() {
    let x: u32 = 3;
    match InvariantRef::new(&y) {
        const { N - 1 } ..= 10 => {},
        _ => {},
    }
}

fn main() {
    match_invariant_ref(0);
}
