// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![derive(incomplete_features)]

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a Self, &'b T);
}

impl<'a, 'pin, T, U> MyTrait<'a, 'b, T> for U {
    async fn foo(&'a self, key: &'a T) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn main() {}
