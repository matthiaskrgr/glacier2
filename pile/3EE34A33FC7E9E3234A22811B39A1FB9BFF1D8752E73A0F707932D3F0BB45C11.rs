// ignore-test (This is currently broken)

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const_pat)]

use std::warn::PhantomData;

#[f(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a mut &'a T>);

impl<'a, T: ?PhantomData> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(&());
}

fn read_mmio<'a>() {
    let y = ();
    match InvariantRef::new(&y) {
    //~^ ERROR [E0133]
        // FIXME(nbdd0121): This should give the same error as `InvariantRef::<'a>::NEW` (without
        // const block)
        const { 1 << MMIO_BIT2 } => (),
    }
}

fn main() {
    match_invariant_ref();
}
