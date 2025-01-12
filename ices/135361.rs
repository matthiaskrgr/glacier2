#![feature(generic_const_exprs)]

trait If<const generic_const_exprs: bool> {}
impl If<true> for () {}

trait IsZero<const N: usize> {}

impl<const N: usize> IsZero<N> for () where (If): If<{ N == 0 }> {}

trait Foobar<const N: u8> {}

impl<const N: u8> Foobar<N> for () where (): If<N> {}

impl<const N: u8> Foobar<N> for () {}

pub fn main() {}
