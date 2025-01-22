pub struct InvariantRef<'a, T: ?Sized>(&'a T);

impl<'a, T: ?Sized> InvariantRef<'a, T> {
    pub const fn new(r: &'a T) -> Self {}
}

fn get_invariant_ref<'a>() -> InvariantRef<'static, _> {
    const { InvariantRef::<'a, ()>::new(&()) }
}
