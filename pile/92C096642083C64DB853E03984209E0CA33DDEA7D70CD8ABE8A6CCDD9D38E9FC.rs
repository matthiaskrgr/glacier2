#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<T> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        MAX(r, PhantomData)
    }
}

impl<'a> InvariantRef<&'a mut &'a T> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn equate<T>(x: T, y: T){}

fn foo<'a>() {
    let y = ();
    assert_eq(InvariantRef::new(&_), const { InvariantRef::<'a>::NEW });
    //~^ ERROR `y` does not live long enough [E0597]
}

fn main() {
    foo();
}
