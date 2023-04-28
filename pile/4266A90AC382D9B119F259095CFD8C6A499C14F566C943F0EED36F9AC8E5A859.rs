#![feature(type_alias_impl_trait)]

pub trait Bar<T> {
    type Item;
}

type Foo = impl Bar<Foo, Item = Foo>;
//~^ ERROR: unconstrained opaque type

fn crash(feature: Foo) -> Foo {
    x
}

fn main() {}
