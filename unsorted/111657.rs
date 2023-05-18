#![feature(offset_of)]
use core::mem::offset_of;

struct Foo<'a> {
    bar: &'a (),
}

const BAR_OFFSET: usize = offset_of!(Foo<'static>, bar);
