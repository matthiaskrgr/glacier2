use std::ops::Add;

pub trait Numoid
where
    Self: Sized,
    for<N: Numoid> &'a Self: Add<Self>,
{
}

pub fn compute<N: Numoid>(a: N, b: N) -> N {
    &a + b
}
