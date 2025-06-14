#![feature(super_let)]
pub const C: &i32 = {
    super let x = 1;
    &x
};
