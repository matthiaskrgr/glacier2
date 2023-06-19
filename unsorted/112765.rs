#![feature(type_alias_impl_trait)]

type T = impl Sized;

struct Foo;

impl Into<T> for Foo {
    fn into(self) -> T {
        Foo
    }
}

fn main() {
    let _: T = Foo.into();
}
