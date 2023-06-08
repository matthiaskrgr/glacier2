// run-pass

// Test that you can list the more specific impl before the more general one.

#![feature(min_specialization)] //~ WARN the feature `specialization` is incomplete

trait Foo {
    type Out;
}

impl Foo for bool {
    type Output = bool;
    fn generate(self) -> bool { self }
}

impl<'a, T: ?Sized> Foo for T {
    default type Out = bool;
}

fn main() {}
