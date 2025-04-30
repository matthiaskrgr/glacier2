@dpytaylo
Description
dpytaylo
opened on Apr 29, 2025
Code

use std::marker::PhantomData;

trait Decode<'a>: Sized {
    type Decoder: Decoder<'a, Self>;
}

trait Decoder<'a, T> {}

struct A {
    b: B,
}

impl<'a> Decode<'a> for A
{
    type Decoder = ADecoder<'a>;
}

pub struct ADecoder<'a> {
    b: <B as Decode<'a>>::Decoder,
}

impl<'a> Default for ADecoder<'a> {
    fn default() -> Self {
        Self {
            b: Default::default(),
        }
    }
}

impl<'a> Decoder<'a, A> for ADecoder<'a> {}

struct NonImplementedDecode;

struct B {
    non_implemented: NonImplementedDecode,
    c: C,
}

impl<'a> Decode<'a> for B
{
    type Decoder = BDecoder<'a>;
}
pub struct BDecoder<'a> {
    non_implemented: <NonImplementedDecode as Decode<'a>>::Decoder,
    c: <C as Decode<'a>>::Decoder,
}
impl<'a> Default for BDecoder<'a> {
    fn default() -> Self {
        Self {
            non_implemented: Default::default(),
            c: Default::default(),
        }
    }
}
impl<'a> Decoder<'a, B> for BDecoder<'a> {}

struct C;

impl<'a> Decode<'a> for C
{
    type Decoder = CDecoder<'a>;
}

pub struct CDecoder<'a> {
    __spooky: PhantomData<&'a ()>,
}

impl<'a> Default for CDecoder<'a> {
    fn default() -> Self {
        Self {
            __spooky: Default::default(),
        }
    }
}

impl<'a> Decoder<'a, C> for CDecoder<'a> {}

fn main() {}
