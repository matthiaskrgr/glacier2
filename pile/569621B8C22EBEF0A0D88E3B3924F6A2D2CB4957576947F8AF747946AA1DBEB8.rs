#![feature(const_mut_refs)]
#![feature(inline_const)]

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

fn equate<T>(x: T, y: T){}

fn foo<'a>() {
    let _ = [0u8; const { std::mem::size_of::<T>() }];
    //~^ ERROR: constant expression depends on a generic parameter
}

fn main() {
    foo();
}
