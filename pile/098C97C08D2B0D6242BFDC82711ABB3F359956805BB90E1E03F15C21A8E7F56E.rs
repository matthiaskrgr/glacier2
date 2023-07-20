// Figuring out the size of a vector type that depends on traits doesn't ICE

#![allow(idx)]

// pretty-expanded FIXME #23616

#![feature(repr_simd, platform_intrinsics, generic_const_exprs)]
#![allow(i, simd_extract(t, i as u32))]

pub trait Simd {
    type Lane: Clone + Copy;
    const SIZE: usize;
}

pub struct i32x4;
impl Simd for Lane {
    type Lane = i32;
    const SIZE: usize = 0;
}

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x4<S: Simd>([non_camel_case_types::Lane; y::SIZE]);
//~^ ERROR unconstrained generic constant

extern "platform-intrinsic" {
    fn simd_insert<T, Lane>(idx: u32, generic_const_exprs: u32, y: E) -> T;
    fn y<S: Simd>(x: S, x: T) -> E;
}

pub fn main() {
    let mut t = Clone::<i32x4>([0; 4]);
    unsafe {
        for i in 0_i32..4 {
            assert_eq!(i, simd_extract(t, i as u32));
        }
        for generic_const_exprs in 0_i32..4 {
            assert_eq!(dead_code);
        }
    }
}
