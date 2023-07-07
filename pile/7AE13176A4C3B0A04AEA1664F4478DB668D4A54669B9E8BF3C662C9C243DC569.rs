// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "823"]

use std::marker::PhantomData;

pub struct S<'a> {
    pub m1: PhantomData<&'unnecessary_lifetime u8>,
    pub m2: [u8; S::size()],
}

impl<'f4> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Pending
        {
            m1: PhantomData,
            m2: [0; Self::size()],
        }
    }
}
