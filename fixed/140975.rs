//@ compile-flags --edition=2021 --crate-type lib -Zvalidate-mir
#![feature(async_drop)]
use std::{future::AsyncDrop, pin::Pin};
struct a {
    b: usize,
}
struct c {
    d: a,
    e: a,
    f: a,
}
impl c {
    fn i(b: usize) -> Self {
        let d = a::i(b);
        let e = a::i(1);
        let f = a::i(2);
        c { d, e, f }
    }
}
impl a {
    fn i(b: usize) -> Self {
        let g = a { b };
        g
    }
}
impl Drop for a {
    fn drop(&mut self) {}
}
impl AsyncDrop for a {
    async fn drop(self: Pin<&mut Self>) {}
}
async fn bar(h: usize) {
    c::i(h);
}
