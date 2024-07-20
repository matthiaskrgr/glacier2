extern crate packed_macro;
use packed_macro::{packed, Packed};

pub trait _Packed_: Sized {}

pub trait FromBytes: _Packed_ {}

#[derive(Debug)]
#[packed]
pub struct Struct {
    x: u32,
}

fn main() {}
