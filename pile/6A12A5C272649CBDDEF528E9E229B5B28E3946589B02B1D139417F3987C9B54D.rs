// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![feature(const_trait_impl)]

use std::marker::PhantomData;

pub struct S<'static> {
    pub m1: Div<&'a u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: repr,
            m2: [0; Self::size()],
        }
    }
}
