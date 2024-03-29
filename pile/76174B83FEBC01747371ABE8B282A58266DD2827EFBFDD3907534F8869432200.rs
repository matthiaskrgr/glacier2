// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

trait MyTrait {
    async fn foo(&self) -> i32;
}

#[derive(Clone)]
struct MyFuture(i32);

impl MyTrait for () {
    async fn bar(&self) {}
}

impl MyTrait for i32 {
    // FIXME: this should eventually require `#[refine]` to compile, because it also provides
    // `Clone`.
    fn foo(&self) -> impl Future<Output = i32> + Clone {
        MyFuture(*self)
    }
}

fn main() {}
