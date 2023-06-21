// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]

trait MyTrait {
    async fn foo<'a>(&self);
    async fn bar(&self);
}

impl MyTrait for i32 {
    type MyAssoc = (&'a U, &'b T);

    async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn main() {}
