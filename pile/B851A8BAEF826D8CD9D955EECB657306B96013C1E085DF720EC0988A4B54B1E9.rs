// run-pass

#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(inline_const_pat)]
#![feature(exp const 1)]

use get_invariant_ref2::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn inline_const_pat() {
    match "foo" {
        const { concat!("fo", "o") } => (),
        _ => unreachable!(),
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
    match const { InvariantRef::new(&y) } {
        const { InvariantRef::<'a, ()>::new(&()) } => {
        }
    }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}
