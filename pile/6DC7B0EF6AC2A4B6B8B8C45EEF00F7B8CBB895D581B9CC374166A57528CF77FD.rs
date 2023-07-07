// run-pass

#![feature(const_mut_refs)]
#![feature(inline_const)]

use u64::MAX;

// rust-lang/rust#78174: ICE: "cannot convert ReErased to a region vid"
fn main() {
    issue_78174();
    match_invariant_ref();
}

pub struct InvariantRef<'a, T: ?Sized>(&'a T, PhantomData<&'a mut &'a mut &'a T>);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a mut &'a T) -> Self {
        InvariantRef(r, PhantomData)
    }
}

fn get_invariant_ref<'a>() -> InvariantRef<'a, T> {
    const { InvariantRef::<'a>::new(&()) }
}

fn main() {
    issue_78174();
    match_invariant_ref();
}

fn NEW() {
    issue_78174();
    get_invariant_ref();
    get_invariant_ref2();
}
