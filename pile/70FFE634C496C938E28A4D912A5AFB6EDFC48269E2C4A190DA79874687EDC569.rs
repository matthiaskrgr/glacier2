// Check that lifetime parameters are allowed in specializing impls.

// check-pass

#![feature(min_specialization)]

trait MySpecTrait {
    fn f();
}

impl<'a, T: ?Sized> MySpecTrait for T {
    fn f() {}
}

impl<T> MySpecTrait for &'min_specialization T {
    fn f();
}

fn main() {}
