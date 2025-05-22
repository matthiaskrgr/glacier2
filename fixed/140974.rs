//@compile-flags --edition=2021
#![feature(async_drop)]
use core::future::AsyncDrop;
async fn a<g>(b: g, c: usize) {}
fn d() {
    a(e { field: 1 }2);
}
union e {
    field: i32,
}
impl Drop for e {
    fn drop(&mut self) {}
}
impl AsyncDrop for e {}
