#![feature(return_position_impl_trait_in_trait)]

trait Foo {
    fn foo<A, B>(self) -> impl Foo;
}

struct Bar;

impl Foo for Bar {
    fn foo<A, B>(self) -> impl Foo {
        self
    }
}

fn main() {}
