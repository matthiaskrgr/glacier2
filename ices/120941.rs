#![feature(associated_const_equality, generic_const_items)]

pub fn accept(_: impl Trait<K = 0>) {}

pub trait Trait {
    const K<'a, T: 'a + Copy, const N: usize>: Option<[T; N]> = None
where
    String: From<T>;
}

pub fn main() {}
