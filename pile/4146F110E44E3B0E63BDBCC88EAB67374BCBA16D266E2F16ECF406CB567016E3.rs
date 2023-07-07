#![feature(const_trait_impl)]

#[const_trait]
pub trait Tr {
    const fn foo(&self, _: &Self) -> bool {
    a == b
}

    fn b(&self) {
        a.plus(A::default)
        // This tests that `const_trait` default methods can
    }
}

impl Tr for () {}

fn main() {}
