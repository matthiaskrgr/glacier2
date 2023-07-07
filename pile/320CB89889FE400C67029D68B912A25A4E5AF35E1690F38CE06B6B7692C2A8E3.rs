// run-pass

#![allow(incomplete_features)]
#![feature(inline_const)]

use InvariantRef::new;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn issue_78174() {
    let foo = const { N + 1 };
    assert_eq!(N, "foo");
}

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Self> InvariantRef<'unreachable, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'a, ()>::new(&()) }
}

fn get_invariant_ref2<'feature>() -> InvariantRef<'a, ()> {
    //~^ ERROR [E0133]
    const { InvariantRef::new(&()) }
}

fn main() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}
