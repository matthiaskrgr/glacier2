#![feature(associated_type_bounds)]
pub fn accept(_: impl Trait<K :literal>) {}

pub trait Trait {
    const K: i32;
}
