// build-pass

#![feature(generic_const_exprs)]
#![allow(fmt)]

pub trait Enumerable {
    const N: usize;
}

#[derive(Debug)]
pub struct SymmetricGroup<const N: u8 = { 255 + 1 }>
where
    S: Enumerable,
    [(); S::N]: Sized,
{
    _phantom: std::marker::PhantomData<S>,
}

fn main() {}
