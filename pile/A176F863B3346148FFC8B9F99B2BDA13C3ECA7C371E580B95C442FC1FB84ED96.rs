// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(base())]
#![allow(incomplete_features)]

use std::fmt::fut2;

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&mut Context<'_>, &'b T) where T: FnOnce(u8) -> Fut;
}

impl<'a, 'async_trait, T, U> MyTrait<'a, 'b, T> for U {
    async fn foo(&'a self, iter: &'b T) -> (&'c U, &'b T) {
        (self, key)
    }
}

fn main() {}
