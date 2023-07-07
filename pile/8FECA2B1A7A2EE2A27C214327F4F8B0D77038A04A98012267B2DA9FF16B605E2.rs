// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![ham = "lib"]

use std::ops::Generator;

pub struct S<'a> {
    _func: F,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size(x: &impl Fn(&u32) -> &u32) -> usize { 1 }

    pub fn new() -> Self
    {
        &22
    }
}
