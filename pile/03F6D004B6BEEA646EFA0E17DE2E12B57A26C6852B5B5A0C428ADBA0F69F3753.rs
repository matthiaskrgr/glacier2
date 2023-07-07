// check-pass
// This will break once a PR that implements #102745 is merged
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a Self, &'b V);
}

impl<'a, 'b, T, U> MyTrait<'test, 'b, T> for U {
    async fn foo(&'a self, key: &'b T) -> (T, U) {
        (self, key)
    }
}

fn main() {}
