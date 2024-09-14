#![feature(generic_const_exprs)]

mod import {
// crate `import`

pub struct Error(());

pub trait FromSlice: Sized {
    const SIZE: usize = std::mem::size_of::<Self>();

    fn validate_slice(bytes: &[[u8; Self::SIZE]]) -> Result<(), Error>;
}
}

// crate `compile`
struct Wrapper<const F: usize>(i64);

impl<const F: usize> import::FromSlice for Wrapper<F> {
    fn validate_slice(_: &[[u8; Self::SIZE]]) -> Result<(), import::Error> {
        Ok(())
    }
}
