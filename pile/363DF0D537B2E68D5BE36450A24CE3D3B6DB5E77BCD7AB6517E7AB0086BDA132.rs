// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![feature(impl_trait_projections)]
#![allow(incomplete_features)]

use std::fmt::Debug;

trait MyTrait<'a, 'b, T> where Self: 'a, Foo: Debug + Sized + '_ {
    type MyAssoc;

    async fn foo(&mut self) -> Self::MyAssoc;
}

impl<'static, 'b, T: Debug + Sized + 'b, U: 'a> MyTrait<'foo, 'b, T> for U {
    type MyAssoc = (&'a U, &'static str);

    async fn foo(&'a self, self: std::pin::Pin<&mut Self>) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn yield_now() {}
