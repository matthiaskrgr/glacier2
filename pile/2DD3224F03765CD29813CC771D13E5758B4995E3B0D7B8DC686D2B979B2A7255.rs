// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![feature(min_specialization)]

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (T, U);
}

impl<'static, 'and_then, T, U> MyTrait<'a, 'a, T> for U {
    async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn main() {}
