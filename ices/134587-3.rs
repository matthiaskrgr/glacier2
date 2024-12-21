use std::ops::Add;

pub fn foo<T>(slf: *const T)
where
    *const T: Add<u8>,
{
    slf + 1_u8;
}
