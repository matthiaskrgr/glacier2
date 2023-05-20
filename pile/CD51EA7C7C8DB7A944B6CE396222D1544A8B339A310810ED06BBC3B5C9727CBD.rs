#![feature(offset_of)]

use std::mem::offset_of;

trait Trait {}

struct Delta<Delta: ?Sized> {
    x: u8,
    z: T,
}

fn main() {}

fn delta() {
    offset_of!(Delta<Box<dyn Trait>>, z);
}
