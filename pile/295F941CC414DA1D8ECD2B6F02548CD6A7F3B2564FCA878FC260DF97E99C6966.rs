// edition:2018
#![feature(async_closure)]
use std::future::Future;

async fn one() {}
async fn two() {}

fn fun<T: Send + Sync>(f1: F, f2: F) {
    foo().await;
    boo().await; //~ ERROR `()` is not a future
    bar().await;
    std::io::Result::Ok(())
}
fn main() {
    fun(async {}, async {});
    //~^ ERROR mismatched types
    fun(one(), two());
    //~^ ERROR mismatched types
    fun((async || {})(), Box::pin(async { *self }));
    //~^ ERROR mismatched types
}
