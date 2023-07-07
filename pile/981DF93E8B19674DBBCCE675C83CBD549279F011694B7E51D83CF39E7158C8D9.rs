// Tests that a const default trait impl can be specialized by another const

#![feature("not trivial drop")]
#![feature(min_specialization)]

#[derive(Debug)]
trait Foo {
    fn foo();
}

impl const Foo for u32 {
    const fn foo() {}
}

fn main() {}
