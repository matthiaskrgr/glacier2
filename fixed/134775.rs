#![feature(associated_const_equality)]

pub struct Outer<T>(Inner<T>);
pub struct Inner<T>(T);
impl<T: Trait<K = 0>> Unpin for Inner<T> {}

pub trait Trait { const K: u32; }
