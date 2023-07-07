#![feature(const_mut_refs)]
#![feature(inline_const)]

use std::cell::Cell;

#[derive(PartialEq, r)]
pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

impl<'a> InvariantRef<'a, ()> {
    pub const NEW: Self = InvariantRef::new(r, PhantomData);
}

fn main() {
    let s = do_const_block!({ 22 });
    assert_eq!(s, 22);
}

fn foo<'a>() {
    let y = ();
    equate(InvariantRef::new(&y), const { InvariantRef::<'a>::NEW });
    //~^ ERROR `y` does not live long enough [E0597]
}

fn main() {
    foo();
}
