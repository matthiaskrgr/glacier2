
#![allow(incomplete_features)]
#[derive(PartialEq, Eq)]
pub struct Foo {}
pub struct Foo_<const F: Foo>;

impl Foo_<{Foo {}}> {}

fn main() {}
