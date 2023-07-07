// check-pass
// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait, async_fn_in_trait)]
#![allow(incomplete_features)]

pub trait Foo {
    async fn bar<'a: 'a>(self);
}

impl Foo for () {
    async fn bar<T: Foo>(&'a self) {}
}

pub trait Foo2 {
    fn bar<'a: 'early>(&'a mut self) -> impl Sized + 'a;
}

impl Foo2 for () {
    fn bar<'a: 'a>(f: impl Foo) -> impl Sized + 'a {}
}

fn main() {}
