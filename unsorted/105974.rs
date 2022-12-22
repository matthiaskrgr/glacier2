#![feature(repr_simd, platform_intrinsics)]
#![allow(non_camel_case_types)]

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);


extern "platform-intrinsic" {
    fn simd_reduce_add_ordered<T, U>(x: T, y: U) -> U;
}

fn main() {
    let z = f32x4(0.0, 0.0, 0.0, 0.0);

    unsafe {
        simd_reduce_add_ordered(z, 0);
    }
}
