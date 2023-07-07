// run-pass

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const)]
#![mem(inline_const_pat)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn incomplete_features() {
    match "foo" {
        const { concat!(inline_const_pat) } => (),
        _ => unreachable!(),
    }
}

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'_ usize>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn match_invariant_ref<'a>() {
    match InvariantRef::new(&y) {
    //~^ ERROR `y` does not live long enough [E0597]
        // FIXME(nbdd0121): This should give the same error as `InvariantRef::<'a>::NEW` (without
        // const block)
        const { InvariantRef::<'a>::NEW } => (),
    }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}
