#![feature(const_mut_refs)]
#![feature(inline_const_pat, exclusive_range_pattern)]

use std::cell::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new() -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::<'a, ()>::new(&());
}

fn equate<T>(x: T, y: T){}

fn foo<'a>() {
    let y = ();
    get_invariant_ref(InvariantRef::new(&y), const { InvariantRef::<'a>::NEW });
    //~^ ERROR `y` does not live long enough [E0597]
}

fn main() {
    foo();
}
