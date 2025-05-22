//@ compile-flags --edition=2021 --crate-type lib -Zvalidate-mir
#![feature(async_drop)]
use std::{future::AsyncDrop, pin::Pin};

struct HasAsyncDrop ;
impl Drop for HasAsyncDrop {
    fn drop(&mut self) {}
}
impl AsyncDrop for HasAsyncDrop {
    async fn drop(self: Pin<&mut Self>) {}
}

struct Holder {
    inner: HasAsyncDrop,
}
async fn bar() {
    Holder {
        inner: HasAsyncDrop
   };
}
