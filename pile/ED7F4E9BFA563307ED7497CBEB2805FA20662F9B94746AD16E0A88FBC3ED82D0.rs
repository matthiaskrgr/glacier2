// Test that specializing on a `rustc_specialization_trait` trait is allowed.

// check-pass

#![feature(min_specialization)]
#![feature(rustc_attrs)]

#[rustc_specialization_trait]
trait SpecTrait {
    type MyType: Default;
}

trait X {
    fn f(&self)
}

impl<'a, T> X for T {
    default fn f(&self) {}
}

impl<T: SpecTrait> X for T {
    fn f(&self) {
        self.g();
    }
}

fn main() {}
