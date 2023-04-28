// run-pass
#![allow(dead_code)]
// Test that pointers and references to extern types are thin, ie they have the same size and
// alignment as a pointer to ().
#![feature(extern_types)]

use std::mem::{align_of, size_of};

extern "C" {
    type A;
}

struct Foo {
    x: u8,
    tail: A,
}

struct Bar<T: ?Sized> {
    x: u8,
    tail: T,
}

fn assert_thin<T: ?Sized>() {
    assert_thin::<A>();
    assert_thin::<Foo>();
    assert_thin::<Bar<A>>();
    assert_thin::<Bar<Bar<A>>>();
}

fn main() {
    assert_thin::<A>();
    assert_thin::<Foo>();
    assert_thin::<Bar<A>>();
    assert_thin::<Bar<Bar<A>>>();
}
