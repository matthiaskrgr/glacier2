#![feature(inherent_associated_types)]

struct Foo<T>(T);

impl Foo<fn(&())> {
    type Assoc = ();
}

fn f(_: for<'a> fn(Foo<fn(&'a ())>::Assoc)) {}
