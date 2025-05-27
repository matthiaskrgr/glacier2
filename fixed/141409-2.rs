// compile-flags -Zmir-enable-passes=+Inline  -Zvalidate-mir  --edition=2021 -Zlint-mir
#![feature(async_drop)]
use std::{
    future::{async_drop_in_place, Future},
    pin::pin, task::Context,
};
fn main() {
    let mut fut_pin = async { async {}.await };

    let drop_fut_unpin = unsafe { async_drop_in_place(&raw mut fut_pin) };
    let drop_fut = pin!(drop_fut_unpin);
    drop_fut.poll(&mut mk());
}
fn mk() -> Context<'static> {
    todo!()
}
