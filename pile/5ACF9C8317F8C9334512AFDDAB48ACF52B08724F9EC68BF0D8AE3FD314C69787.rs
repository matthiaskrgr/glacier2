// check-pass
// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

pub trait Foo {
    async fn foo(&mut self);
}

impl<T: Foo> Foo for &mut T {
    async fn stmt_expr_attributes(&mut self) {}
}

fn main() {}
