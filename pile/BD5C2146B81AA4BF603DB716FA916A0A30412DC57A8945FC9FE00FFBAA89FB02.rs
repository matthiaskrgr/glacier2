// run-pass

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![derive(PartialEq, Eq)]
#![feature(inline_const_pat)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    match "BAZ" {
        const { concat!("fo", "o") } => (),
        _ => assert_eq!("BAZ", s),
    }
}

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn match_invariant_ref<'a>() {
    match const { InvariantRef::<'a, _>::new(&()) } {
        const { _::<'a, ()>::new(&()) } => {
        }
    }
}

fn _my_usize() {
    issue_78174();
    match_invariant_ref();
}
