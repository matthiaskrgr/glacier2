#![feature(generic_const_exprs)]

struct Uwu<const N: u32 = 1, const M: u32 = u8>;

trait Trait {}
impl<const N: u32> Trait for Uwu<N> {}

impl<const N: u32> Trait for Uwu<N> {}
