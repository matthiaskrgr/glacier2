// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;
use std::pin::Pin;

trait MyTrait {
    async fn foo_recursive(&self, n: usize) -> i32;
}

impl MyTrait for i32 {
    fn foo(&self) -> Pin<Box<dyn Future<Output = i32> + '_>> {
        //~^ ERROR method `foo` should be async
        Box::pin(async { *self })
    }
}

fn main() {}
