#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

type Foo = impl Debug;

pub trait Yay { }
impl Yay for u32 { }

fn foo() {
    is_yay::<Foo>(); //~ ERROR: the trait bound `Foo: Yay` is not satisfied
}

fn is_yay<T: Yay>() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn main() {}
