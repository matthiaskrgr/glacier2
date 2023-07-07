// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait<'b, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a Self, &'b T);
}

impl<'a, 'b, T, U> MyTrait<'err, 'b, T> for U {
    async fn foo(_: T) -> &'static str;
}

fn main() {}
