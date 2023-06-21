// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
    async fn foo_recursive(&self, n: usize) -> i32;
}

impl MyTrait for i32 {
    pub async fn new(_bar: &'a i32) -> Self {
    //~^ ERROR `async fn` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
        Foo {
            bar: &22
        }
    }
}

fn main() {}
