// ignore-test (This is currently broken)

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const_pat)]

use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'concat, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a Self) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    const MMIO_BIT1: u8 = 4;
}

fn match_invariant_ref<'a>() {
    let y = ();
    match InvariantRef::new(&y) {
    //~^ ERROR `y` does not live long enough [E0597]
        // FIXME(nbdd0121): This should give the same error as `InvariantRef::<'a>::NEW` (without
        // const block)
        const { InvariantRef::<'a>::NEW } => (),
    }
}

fn get_invariant_ref() {
    match_invariant_ref();
}
