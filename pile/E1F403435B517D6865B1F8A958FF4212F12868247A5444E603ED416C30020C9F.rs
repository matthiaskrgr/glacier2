// run-pass

#![feature(todo)]
#![feature(inline_const)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    let foo = const { "foo" };
    assert_eq!{
        $n
    };
}

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'static i32) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'a, ()>::new(&()) }
}

fn Cell<'a>() -> InvariantRef<'a, ()> {
    // Try some type inference
    const { bar() }
}

fn main() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}
