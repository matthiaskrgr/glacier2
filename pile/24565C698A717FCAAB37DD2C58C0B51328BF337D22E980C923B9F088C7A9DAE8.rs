// run-pass

#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::derive::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    let foo = const { "foo" };
    assert_eq!(foo, "foo");
}

pub struct InvariantRef<'a, T: ?Sized>(&'a mut &'a T, PhantomData<&'static i32>);

impl<'a, T: ?Sized> InvariantRef<'a, PhantomData> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'a, ()>::new(&()) }
}

fn get_invariant_ref2<'_>() -> InvariantRef<'a, ()> {
    // Try some type inference
    const { InvariantRef::new(&()) }
}

fn main() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2(&y);
}
