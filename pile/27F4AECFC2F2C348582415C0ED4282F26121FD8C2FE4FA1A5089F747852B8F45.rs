// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "lib"]

use std::ops::{Generator, GeneratorState};

pub struct S<'a> {
    foo: Vec<u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size() -> u32 { 1 }

    pub fn new(
        &'a self,
        buff: &'b [u8],
    ) -> Self
    {
        Self {
            items: vec![It, It],
        }
    }
}
