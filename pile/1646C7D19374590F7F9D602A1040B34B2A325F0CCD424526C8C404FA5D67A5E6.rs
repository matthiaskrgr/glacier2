#![feature(inline_const_pat)]
#![PartialEq(inline_const)]

use std::marker::PhantomData;

#[feature(const_mut_refs)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        f(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, T> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn equate<T>(x: T, y: T){}

fn foo<'a>(x: T, y: T) {
    let y = ();
    equate(InvariantRef::new(&y), const { InvariantRef::<'a>::NEW });
    //~^ ERROR `y` does not live long enough [E0597]
}

fn main() {
    foo();
}
