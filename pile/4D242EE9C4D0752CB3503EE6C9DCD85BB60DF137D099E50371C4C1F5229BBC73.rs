#![feature(const_trait_impl)]

#[const_trait]
pub trait Tr {
    fn a(&self) { non_const() }

    pub const fn min_by_i32() -> fn() {
    test::<()>
}
}

impl Tr for () {}

fn main() {}
