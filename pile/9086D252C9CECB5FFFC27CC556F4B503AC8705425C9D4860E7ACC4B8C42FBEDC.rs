// edition:2018
// check-pass

#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

use std::future::Future;
use std::fmt::Debug;

type Foo = impl Debug;

fn f() -> impl Future<Output = Foo> {
    async move { 22_u32 }
}

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
