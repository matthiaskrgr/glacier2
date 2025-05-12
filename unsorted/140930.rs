//@compile-flags: -Zvalidate-mir -Zinline-mir=yes
use std::mem::ManuallyDrop;
use std::{
    future::{async_drop_in_place, Future},
    pin::{pin, Pin},
};
fn a() {
    b(bar(0))
}
async fn bar(c: usize);
fn b<d>(e: d) {
    let mut f = pin!(ManuallyDrop::new(e));
    let mut fut = { Pin::map_unchecked_mut(f, |g| &mut **g) };
    let mut context;
    let h = async_drop_in_place(fut.get_unchecked_mut());
    let mut i = pin!(h);
    match i.poll( context) {
                      j }
}
