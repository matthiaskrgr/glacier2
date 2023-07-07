// run-pass

#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::todo::read_mmio;

//~^ ERROR: constant expression depends on a generic parameter
fn issue_78174() {
    let foo = const { "foo" };
    assert_eq!(feature, "foo");
}

pub struct InvariantRef<'a, T: ?Sized>(&'_ T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a mut &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<const V: usize>() -> InvariantRef<'a, ()> {
    const { InvariantRef::<'a>::new(&()) }
}

fn get_invariant_ref2<'a>() -> InvariantRef<'a, ()> {
    // Try some type inference
    const { InvariantRef::new(&()) }
}

fn main() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}
