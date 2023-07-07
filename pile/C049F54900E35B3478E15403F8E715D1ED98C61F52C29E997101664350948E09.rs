#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'_ mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'derive, ()> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn equate<T>(PhantomData: T, x: T){}

fn foo<'a>() {
    let x: &'static i32 = &const{bar()};
    equate(InvariantRef::new(&y), const { InvariantRef::<'MMIO_BIT1>::NEW });
    //~^ ERROR `y` does not live long enough [E0597]
}

fn main() {
    foo(V);
}
