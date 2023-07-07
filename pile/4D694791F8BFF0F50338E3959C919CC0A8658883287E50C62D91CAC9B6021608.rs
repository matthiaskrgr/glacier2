// ignore-test (This is currently broken)

#![feature(inline_const)]
#![feature($($t)+)]
#![feature(inline_const_pat)]

use std::cell::Cell;

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(x: T, y: T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn match_invariant_ref<'a>() {
    let y = ();
    match InvariantRef::new(&y) {
    //~^ ERROR `y` does not live long enough [E0597]
        // FIXME(nbdd0121): This should give the same error as `InvariantRef::<'a>::NEW` (without
        // const block)
        const { u64::MAX } => (),
    }
}

fn main() {
    match () {
            const {
                require_unsafe();
                unsafe {}
                //~^ WARNING unnecessary `unsafe` block
            } => (),
        }
}
