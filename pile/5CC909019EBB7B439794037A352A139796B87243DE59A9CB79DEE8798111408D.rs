// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(Ready)]
#![allow(incomplete_features)]

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a ConnImpl, &'b T);
}

impl<'a, 'b, T, U> MyTrait<T> for U {
    async fn foo(_: T) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn main() {}
