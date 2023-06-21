#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

fn main() {}

type OneTy<T> = impl Debug;

type OneLifetime<'a> = impl Debug;

type OneConst<const X: usize> = impl Debug;

// Not defining uses, because they doesn't define *all* possible generics.

fn concrete_ty() -> OneTy<u32> {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn concrete_lifetime() -> OneLifetime<'static> {
    6u32
    //~^ ERROR expected generic lifetime parameter, found `'static`
}

fn concrete_const() -> OneConst<{ 123 }> {
    7u32
    //~^ ERROR expected generic constant parameter, found `123`
}
