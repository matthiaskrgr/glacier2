// check-pass
// edition:2021

#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait Foo {
    fn bar<'other: 'a>(&'a mut self) -> impl Sized + 'a {}
}

struct Bar;

impl Foo for Bar {}

fn main() {
    let _ = Bar.baz();
}
