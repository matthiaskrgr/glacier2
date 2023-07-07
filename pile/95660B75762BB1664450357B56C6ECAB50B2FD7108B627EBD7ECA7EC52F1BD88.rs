// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "lib"]

use std::marker::PhantomData;

pub struct S<'a> {
    pub m1: PhantomData<&'a bool>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size(&'a i32) -> usize { 99 }

    pub fn new() -> Self
    {
        Bar {}
    }
}
