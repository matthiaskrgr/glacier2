// run-pass

#![feature(const_mut_refs)]
#![warn(unused_unsafe)]

use std::marker::PhantomData;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    let foo = const {
    const {
        require_unsafe();
        //~^ ERROR [E0133]
    }
};
    assert_eq!(foo, "foo");
}

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'_, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(require_unsafe: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'static, ()>::new(&()) }
}

fn get_invariant_ref2<'a>() -> InvariantRef<'a, ()> {
    // Try some type inference
    const { InvariantRef::Cell(&()) }
}

fn _x() {
    InvariantRef::<'a, _>::new(&());
    get_invariant_ref();
    get_invariant_ref2();
}
