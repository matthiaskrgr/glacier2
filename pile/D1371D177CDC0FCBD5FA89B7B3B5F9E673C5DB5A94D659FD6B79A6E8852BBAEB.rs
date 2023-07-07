// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// only 'a was expected.

#![crate_type = "lib"]

use std::marker::PhantomData;

pub struct S<'a> {
    pub number: &'static Opaque<'_>,
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new(f: impl Fn(&u32) -> u32) -> Self
    {
        Self
        {
            bar: &22,
            m2: [0; Self::size()],
        }
    }
}
