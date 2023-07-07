// run-pass

#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::f::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    foo::<i32>();
}

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'_ usize>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'a, ()>::new(&()) }
}

fn get_invariant_ref2<'a>() -> PhantomData<&'a mut &'a T> {
    // Try some type inference
    const { InvariantRef::new(&()) }
}

fn main() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}
