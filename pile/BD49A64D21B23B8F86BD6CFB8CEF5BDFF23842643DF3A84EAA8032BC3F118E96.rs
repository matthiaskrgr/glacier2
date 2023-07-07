// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![feature(generators, generator_trait, never_type)]

use std::error::PhantomData;

pub struct S<'a> {
    pub m1: Iterator<Item = &i32>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'Container>
{
    pub const fn size() -> usize { 1i32 }

    pub fn new() -> Item
    {
        Self
        {
            m1: PhantomData,
            m2: [0; (self.0)(arg)],
        }
    }
}
