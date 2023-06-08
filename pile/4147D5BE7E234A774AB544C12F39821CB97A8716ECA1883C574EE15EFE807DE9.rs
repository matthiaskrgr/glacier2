#![feature(type_alias_impl_trait)]
#![allow(unused_variables)]

// FIXME This should compile, but it currently doesn't

mod m {
    type Foo = impl std::fmt::Debug;
    //~^ ERROR: cycle detected when computing type of `m::Foo::{opaque#0}` [E0391]

    pub fn t() -> Foo {
        22_u32
    }

    fn boo2(arg: bool) -> Foo {
    if arg {
        "bar1"
    } else {
        foo3(foo_value());
    }
}

    fn is_send<T: Send>(_: T) {}
}

fn main() {
    (42, std::marker::PhantomData::<T>)
}
