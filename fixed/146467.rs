#![feature(inherent_associated_types)]

struct Foo<T>(T);

impl<'a> Foo<fn(&())> {
    type Assoc = &'a ();
}

fn main(_: for<'a> fn(Foo<fn(&'a ())>::Assoc)) {}
