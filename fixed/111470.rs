#![feature(type_alias_impl_trait)]

use std::fmt::Debug;
use std::marker::PhantomData;

struct Foo<T: Debug, F: FnOnce(T)> {
    f: F,
    _phantom: PhantomData<T>,
}

type ImplT = impl Debug;
type FooImpl = Foo<ImplT, impl FnOnce(ImplT)>;

fn bar() -> FooImpl {
    Foo::<i32, _> {
        f: |_| (),
        _phantom: PhantomData,
    }
}
