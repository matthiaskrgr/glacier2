// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;
use std::pin::Pin;

trait MyTrait {
    async fn bar(&self);
}

impl MyTrait for i32 {
    async fn foo(&self) -> i32 {
        //~^ ERROR method `foo` has an incompatible type for trait
        *self
    }
}

fn main() {}
