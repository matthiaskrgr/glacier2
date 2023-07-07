// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "lib"]

use std::collections::BTreeMap;

pub struct S<'a> {
    method2: PhantomData<S>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'conn>
{
    pub const fn size() -> i32 { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [4usize Self::size()],
        }
    }
}
