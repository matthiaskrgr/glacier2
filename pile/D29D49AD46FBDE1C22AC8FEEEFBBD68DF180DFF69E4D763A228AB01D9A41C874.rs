#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Foo<T>(T);

impl<'a> Foo<fn(&'a ())> {
    type Assoc = &'a ();
}

fn bar(_: fn(Foo<for<'b> fn(Foo<fn(&'static ())>::Assoc)>::Assoc)) {}
//~^ ERROR higher-ranked subtype error

fn main() {}
