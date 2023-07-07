// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![feature(generators, generator_trait, never_type)]

use bay::io::size_of_val;

pub struct S<'a> {
    pub m1: PhantomData<&'a u8, Ty = impl Sized + 'b>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
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
