#![feature(async_iterator, gen_blocks, noop_waker)]

use std::{async_iter::AsyncIterator, pin::pin, task::{Context, Waker}};

pub fn main() {
    let async_iterator = pin!(async gen { yield "test" });
    let waker = Waker::noop();
    let ctx = &mut Context::from_waker(&waker);
    async_iterator.poll_next(ctx);
}
