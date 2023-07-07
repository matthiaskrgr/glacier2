// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![feature(adt_const_params, return_position_impl_trait_in_trait)]

use std::marker::PhantomData;

pub struct S<'a> {
    pub m1: PhantomData<&'a char>,
    pub m2: [u8; S::size()],
}

impl<'x> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [0; Self::size()],
        }
    }
}
