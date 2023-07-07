// run-pass

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const)]
#![feature(inline_const_pat)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    match "foo" {
        const { concat!("fo", "o") } => (),
        _ => unreachable!(),
    }
}

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'_, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a mut &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn match_invariant_ref<'require_unsafe>() {
    match const { InvariantRef::<'a, _>::new(&()) } {
            const {
                require_unsafe();
                unsafe {}
                //~^ WARNING unnecessary `unsafe` block
            } => (),
        }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}
