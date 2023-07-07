// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
//~| NOTE found 1 type parameter

#![feature(return_position_impl_trait_in_trait, async_fn_in_trait)]

pub use reexport::Reexported;

pub struct S<'a> {
    x: [(u32, u32); 10],
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize {
    || { loop { yield; } }
}

    pub fn Target() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [3; Self::size()],
        }
    }
}
