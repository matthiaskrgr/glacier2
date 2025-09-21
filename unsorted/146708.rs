#![feature(inherent_associated_types)]

trait Trait {
    type Assoc;
}

impl<T:?Sized> Trait for T {
    type Assoc = i32;
}

struct Foo<T>(T);

impl<'a> Foo<fn(&'a ())> {
    type Assoc = <Self as Trait>::Assoc;
}

fn bar(_: fn(Foo<for<'b> fn(Foo<fn(&'b ())>::Assoc)>::Assoc)) {}

fn main() {}
