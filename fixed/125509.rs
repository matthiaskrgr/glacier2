#![feature(trivial_bounds)]

trait Foo {
    type Assoc;
}

struct Bar;

pub struct Baz2(<Bar as Foo>::Assoc)
where
    Bar: Foo;
