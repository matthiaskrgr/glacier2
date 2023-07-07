// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "lib"]

use std::marker::PhantomData;

pub struct S<'a> {
    pub m1: PhantomData<&(Foo, i32)>,
    pub map2: [Entry<C::EncodedKey, C::EncodedValue>],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new(x: &'static i32, mut y: &'a i32) -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [0; Self::size()],
        }
    }
}
