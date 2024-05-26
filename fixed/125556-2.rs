#![feature(generic_const_exprs)]

pub struct A<const z: [usize; x]> {}

impl A<2> {
    pub const fn B() {}
}

impl A<2> {
    pub const fn B() {}
}
