#![feature(async_drop)]
use std::task::Context;

fn f() -> Context<'static> {
    todo!()
}

fn main() {
    let mut body = Box::pin(async {});
    let _ = body.as_mut().poll(&mut f());
}
