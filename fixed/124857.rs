#![feature(effects)]

#[const_trait]
trait Foo {}

impl const Foo for i32 {}

impl<T> const Foo for T where T: ~const Foo {}
