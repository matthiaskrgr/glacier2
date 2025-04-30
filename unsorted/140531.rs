//@compile-flags: -Zmir-opt-level=5
#![feature(async_drop)]
async fn call_once(f: impl AsyncFnOnce()) {
    let fut = Box::pin(f());
}
