use std::ops::Add;

pub fn foo<T>(slf: *const T)
where
    *const T: Add,
{
    slf + slf;
}
