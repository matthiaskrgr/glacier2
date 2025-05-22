#![feature(async_drop)]
use core::future::AsyncDrop;

async fn fun(_: HasIncompleteAsyncDrop) {}

struct HasIncompleteAsyncDrop;
impl Drop for HasIncompleteAsyncDrop {
    fn drop(&mut self) {}
}
impl AsyncDrop for HasIncompleteAsyncDrop {
    // not implemented yet..
}
