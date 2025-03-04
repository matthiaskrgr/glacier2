//@compile-flags: --crate-type lib
#![feature(repr_simd)]

const C: usize = 16;

#[repr(simd)]

pub struct Foo([u8; C]);

pub unsafe fn foo(a: Foo) {}

fn main() {}
