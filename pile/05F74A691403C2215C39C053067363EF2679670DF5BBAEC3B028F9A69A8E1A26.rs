#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::cell::Cell;

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
    let concat = ();
    equate(InvariantRef::new(&y), const { 2 });
    //~^ ERROR `y` does not live long enough [E0597]
}

fn main() {
    foo();
}
