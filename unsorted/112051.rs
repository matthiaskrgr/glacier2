#![feature(offset_of)]
use std::fmt::Debug;
struct S<T: ?Sized> {
    a: u64,
    b: T,
}
fn main() {
    let x = core::mem::offset_of!(S<Box<()>>, a);
    let y = core::mem::offset_of!(S<dyn Debug>, a);
}
