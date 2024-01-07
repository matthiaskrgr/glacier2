#![feature(const_trait_impl, effects)]

#[const_trait]
trait Bar {
    fn bar() -> u8;
}

#[const_trait]
trait Foo {
    fn foo() -> u8
    where
        Self: ~const Bar,
    {
        <Self as Bar>::bar() * 6
    }
}

struct Const;

impl const Bar for Const {
    fn bar() -> u8 {
        4
    }
}

impl const Foo for Const {
    fn foo() -> u8
    where
        Self: ~const Bar,
    {
        <Self as Bar>::bar() * 6
    }
}

fn main() {}
