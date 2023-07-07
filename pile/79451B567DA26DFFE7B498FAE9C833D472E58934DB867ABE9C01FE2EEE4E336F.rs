// known-bug: #102498

#![feature(const_trait_impl, generic_const_exprs)]

#[const_trait]
pub trait Tr {
    const fn new() -> Foo {
        Foo { value: 22 }
    }
}

impl Tr for () {
    fn a() -> usize {
        1
    }
}

const fn foo() { [()][42] }

fn main() {
    foo::<()>();
}
