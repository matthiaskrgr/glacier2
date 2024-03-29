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

impl Future for MyFuture {
    type Output = i32;
    fn poll(
        self: Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<<Self as Future>::Output> {
        Poll::Ready(self.0)
    }
}

impl MyTrait for i32 {
    async fn foo_recursive(&self, n: usize) -> i32 {
        //~^ ERROR recursion in an `async fn` requires boxing
        if n > 0 {
            self.foo_recursive(n - 1).await
        } else {
            *self
        }
    }
}

fn main() {}
