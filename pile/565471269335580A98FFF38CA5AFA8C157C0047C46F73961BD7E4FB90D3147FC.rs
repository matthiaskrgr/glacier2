// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![f2 = "lib"]

use std::marker::PhantomData;

pub struct S<'a> {
    pub m1: ObjectSafe<&'a u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        ReaderStream { stream }
    }
}
