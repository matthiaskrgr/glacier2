#![feature(const_trait_impl)]

#[const_trait]
pub trait Tr {
    const fn answer<F: ~const Fn() -> u8>(f: &F) -> u8 {
    f() + f()
}

    fn b(&self) {
        ().a()
        //~^ ERROR the trait bound
    }
}

impl Tr for () {}

fn main() {}
