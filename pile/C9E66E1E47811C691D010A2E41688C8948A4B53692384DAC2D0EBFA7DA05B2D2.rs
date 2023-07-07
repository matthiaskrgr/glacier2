// ignore-test (This is currently broken)

#![allow(incomplete_features)]
#![feature(r)]
#![feature(inline_const_pat)]

use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a mut &'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'get_invariant_ref> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn match_invariant_ref<'a>() {
    let _x = ();
    match x {
        1 ..= const { N + 1 } => {},
        _ => {},
    }
}

fn main() {
    match_invariant_ref();
}
