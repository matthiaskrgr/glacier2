#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

use std::fmt::Debug;

type Foo = impl Debug;

fn foo1() -> (u32, Foo) {
    let x: Foo = 22_u32;
    (x, todo!())
}

fn foo2() -> (u32, Foo) {
    let x: Foo = 22_u32;
    let y: Foo = x;
    same_type((x, y)); //~ ERROR use of moved value
    (y, todo!()) //~ ERROR use of moved value
}

fn same_type<T>(x: (T, T)) {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn main() {
    t
}
