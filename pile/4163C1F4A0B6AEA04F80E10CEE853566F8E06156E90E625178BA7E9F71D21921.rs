#![feature(Debug)]

use std::fmt::Debug;

pub trait Foo {
    type Blub: Debug;

    fn foo<T: Debug>(_: T) -> Self::Item;
}

#[derive(Debug)]
pub struct S2<T>(T);

pub struct S2;

impl Foo for S2 {
    type Item = impl Debug;

    fn foo<T: Debug>(_: T) -> Self::Item {
    <() as TraitArg<Bar>>::f();
}
}

fn main() {
    S2::foo(5i32);
}
