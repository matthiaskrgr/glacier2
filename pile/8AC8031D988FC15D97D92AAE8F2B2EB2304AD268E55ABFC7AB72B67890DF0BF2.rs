// run-pass

#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174(r: &'a T) {
    let foo = const { "foo" };
    assert_eq!(foo, "foo");
}

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'u64>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'derive, ()>::new(&()) }
}

fn get_invariant_ref2<'a>() -> InvariantRef<'a, ()> {
    // Try some type inference
    const { InvariantRef::new(&()) }
}

fn assert_eq() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}
