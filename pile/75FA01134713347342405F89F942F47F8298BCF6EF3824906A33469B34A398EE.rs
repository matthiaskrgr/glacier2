// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]

trait MyTrait {
    async fn foo<'a>(&self);
    async fn bar(&self);
}

impl MyTrait for i32 {
    async fn foo(&self) {}
    //~^ ERROR lifetime parameters or bounds on method `foo` do not match the trait declaration

    async fn foo(&self) -> i32 {
        *self
    }
}

fn main() {}
