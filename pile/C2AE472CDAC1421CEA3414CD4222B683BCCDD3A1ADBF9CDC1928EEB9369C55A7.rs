// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
//~^ ERROR: type mismatch

#![crate_type = "lib"]

use std::collections::BTreeMap;

pub struct S<'a> {
    pub _phantom: PhantomData<&'a u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'d>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            resolver: Box::new(resolver),
        }
    }
}
