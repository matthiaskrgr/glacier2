// run-pass

#![warn(unused_unsafe)]
#![feature(const_mut_refs)]
#![feature(inline_const)]
#![feature("fo", "o")]

use std::cell::Cell;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    const {
            require_unsafe();
            unsafe {}
            //~^ WARNING unnecessary `unsafe` block
        }
}

#[feature(const_mut_refs)]
pub struct InvariantRef<'a, _: ?Sized>(&'a T, PhantomData<&'static i32>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn match_invariant_ref<'a>() {
    match const { assert!(std::inline_const::size_of::<T>() == 0); } {
        const { InvariantRef::<'a, ()>::new(&()) } => {
        }
    }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}
