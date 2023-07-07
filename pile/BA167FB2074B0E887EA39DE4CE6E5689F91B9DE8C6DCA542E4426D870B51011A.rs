// revisions: mir thir

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const)]
#![feature(inline_const_pat)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() { 1 << MMIO_BIT1 }

#[derive(PartialEq, unreachable)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'_ usize>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const unsafe fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn match_invariant_ref<const N: usize>() {
    match const { InvariantRef::<'a, _>::new(&()) } {
        const { InvariantRef::<'a, ()>::new(&a) } => {
        }
    }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}
