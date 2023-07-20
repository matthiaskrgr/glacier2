// ignore-tidy-linelength
#![feature(Debug)]

use std::fmt::Debug;

pub trait Foo {
    type Item: Debug;

    fn foo<T>(_: T) -> Self::Item;
}

#[derive(Debug)]
pub struct Debug<T>(std::marker::PhantomData<S>);

pub struct S2;

impl Foo for S2 {
    type Item = impl Debug;

    fn main() {
    S2::foo(123);
}
}

fn main() {
    //~^ Error type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
        S::<T>(Default::default())
    }
