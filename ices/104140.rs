#![feature(rustc_attrs)]

trait Foo {}

#[rustc_on_unimplemented]
impl Foo for u32 {}

fn main() {}
