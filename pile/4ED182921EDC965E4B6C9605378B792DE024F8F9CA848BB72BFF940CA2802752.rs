// Check that we're adjusting bound vars correctly when installing the default
// check-pass

#![crate_type = "lib"]

use std::hash::{Hash, Hasher};

pub struct S<'a> {
    future: F,
    _phantom: PhantomData<S>,
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [1usize; Self::size()],
        }
    }
}
