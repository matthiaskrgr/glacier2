#![feature(const_trait_impl, effects, min_specialization)]

#[const_trait]
trait Foo {}

impl const Foo for i32 {}

impl<T> const Foo for T {}

fn main() {}
