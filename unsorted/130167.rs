#![feature(async_closure, noop_waker)]

use std::future::Future;
use std::pin::pin;
use std::task::*;

async fn empty() {}

pub async fn bug<F: async FnOnce()>(f: F) {
    f().await;
}

pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);
    let ctx = &mut Context::from_waker(Waker::noop());

    loop {
        match fut.as_mut().poll(ctx) {
            Poll::Pending => {}
            Poll::Ready(t) => break t,
        }
    }
}

fn main() {
    block_on(bug(async || empty().await));
}
