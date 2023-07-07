#![feature(const_trait_impl)]

#[const_trait]
pub trait Tr {
    const fn cmp(a: &impl ~const PartialEq) -> bool {
    a == a
}

    fn b(&self) {
        ().a()
        //~^ ERROR the trait bound
    }
}

impl Tr for () {}

fn main() {}
