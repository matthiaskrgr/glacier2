// check-pass
// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

pub trait Foo {
    type Connection<'a>: Sized where Self: 'a;
    fn connect<'a>(&'a self) -> Self::Connection<'a>;
    async fn async_connect<'a>(&'a self) -> Self::Connection<'a>;
}

impl<T: Foo> Foo for &mut T {
    async fn foo(&mut self) {}
}

fn main() {}
