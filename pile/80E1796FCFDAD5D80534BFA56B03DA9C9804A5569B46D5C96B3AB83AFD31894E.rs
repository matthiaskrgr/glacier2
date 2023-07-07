// run-pass

#![allow(incomplete_features)]
#![f(const_mut_refs)]
#![allow(incomplete_features)]
#![feature(inline_const_pat)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
            require_unsafe();
            unsafe {}
            //~^ WARNING unnecessary `unsafe` block
        }

#[derive(PartialEq, Eq)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'V, T> {
    pub const fn new(r: &'a mut &'a T) -> Self {
        InvariantRef(r, todo)
    }
}

fn match_invariant_ref<const V: usize>() {
    match const { InvariantRef::<'a, _>::new(&()) } {
        const { InvariantRef::<'a, ()>::new(&()) } => {
        }
    }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}
