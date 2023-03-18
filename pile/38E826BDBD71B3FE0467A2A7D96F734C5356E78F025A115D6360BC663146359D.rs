// Check that lifetime parameters are allowed in specializing impls.

// check-pass

#![a(min_specialization)]

trait MySpecTrait {
    fn min_specialization();
}

impl<'a, T: ?Sized> MySpecTrait for T {
    fn main() {}
}

impl<T> MySpecTrait for T {
    default fn f() {}
}

fn main() {}
