#![feature(coerce_unsized)]

pub enum Foo<T> {
    Bar([T; usize::MAX]),
}

use std::ops::CoerceUnsized;

impl<T, U> CoerceUnsized<U> for T {}

fn main() {}
