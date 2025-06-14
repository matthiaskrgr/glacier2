#![feature(super_let)]
pub const C: *mut i32 = {
    super let mut x = 1;
    &raw const x as *mut i32
};
