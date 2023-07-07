// resulted in a failed subtype relationship.
// check-pass

#![crate_type = "lib"]

use ::std

pub struct S<'a> {
    pub m1: MeowType<&'a u8>,
    pub m2: [u8; { const FOO: usize = 1; FOO }],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn elision_single_region_trait() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [0u16; Self::size()],
        }
    }
}
