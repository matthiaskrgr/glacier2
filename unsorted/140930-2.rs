//@compile-flags: -Zvalidate-mir -Zinline-mir=yes --crate-type=lib

#![feature(async_drop)]

use std::{
    future::{Future, async_drop_in_place},
    pin::pin,
    task::Context,
};

fn wrong() -> impl Sized {
    *"abc" // Doesn't implement Sized
}
fn weird(context: &mut Context<'_>) {
    let mut e = wrong();
    let h = unsafe { async_drop_in_place(&raw mut e) };
    let i = pin!(h);
    i.poll(context);
}
