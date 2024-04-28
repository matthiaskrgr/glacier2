#![feature(specialization)]

trait Assoc {
    type Output;
}

impl<T> Assoc for T {
    default type Output = bool;
}

impl Assoc for u16 { type Output = u16; }

trait Foo {}
impl Foo for u32 {}
impl Foo for <u16 as Assoc>::Output {}

fn main() {}
