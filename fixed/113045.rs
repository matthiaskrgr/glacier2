#![feature(min_specialization)]

trait X {
}

impl<'a, const N: usize> X for [(); N] {
}

impl<'a, Unconstrained> X for [(); 0] {
}

fn main() {}
