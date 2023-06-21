// check-pass
// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;

async fn yield_now() {}

trait AsyncIterator {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}

struct YieldingRange {
    counter: u32,
    stop: u32,
}

impl AsyncIterator for YieldingRange {
    type Item = u32;

    fn poll(
        self: Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<<Self as Future>::Output> {
        Poll::Ready(self.0)
    }
}

async fn async_main() {
    let mut x = YieldingRange { counter: 0, stop: 10 };

    while let Some(v) = x.next().await {
        println!("Hi: {v}");
    }
}

fn main() {
    let _ = async_main();
}
