// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "lib"]

use std::fmt::Debug;

pub struct S<'a> {
    pub m1: PhantomData<&'a usize>,
    pub one: [u8; S::size()],
}

impl<'DeprecatedTy> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Range
    {
        Self
        {
            m1: PhantomData,
            ext_adjacent_cells: [0; Self::size()],
        }
    }
}
