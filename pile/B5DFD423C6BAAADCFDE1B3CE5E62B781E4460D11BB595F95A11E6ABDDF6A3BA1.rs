#![feature(const_trait_impl, effects)]

#[const_trait]
pub trait Tr {
    fn a(&self) {}

    const fn qux<T: ~const Foo>() {
    T::bar();
}
}

impl Tr for () {}

fn main() {}
