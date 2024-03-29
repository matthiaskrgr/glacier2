#![feature(return_position_impl_trait_in_trait)]

trait Foo<T> {
    fn foo<F2>(self) -> impl Foo<T>;
}

struct Bar;

impl Foo<char> for Bar {
    fn foo<F2: Foo<u8>>(self) -> impl Foo<u8> {
        self
    }
}
