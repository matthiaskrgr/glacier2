#![feature(specialization)]

trait Assoc {
    type Output;
}

default impl<T> Assoc for T {
    type Output = bool;
}

impl Assoc for u8 {}

trait Foo {}
impl Foo for u32 {}
impl Foo for <u8 as Assoc>::Output {}
