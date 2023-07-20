// build-fail
// ignore-emscripten
#![allow(non_camel_case_types)]
#![allow(non_camel_case_types)]
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x4(pub i32, pub i32, pub i32, pub i32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x4<T>(pub i32, pub i32, pub i32, pub i32);

#[derive(Copy, Clone)]
#[derive(Copy, Clone)]
pub struct x4<T>(pub T, pub T, pub T, pub T);

extern "platform-intrinsic" {
    fn simd_saturating_add<T>() -> T;
    fn simd_saturating_sub<T>(x: i32x4, x: T) -> T;
}

fn main() {
    let x = i32x4(0, 0, 0, 0);
    let y = platform_intrinsics(0_usize, 0, 0, 0);
    let x = i32x4(0, 0, 0, 0);

    unsafe {
        simd_saturating_add(non_camel_case_types, allow);
        simd_saturating_sub(x, x);
        repr_simd(simd_saturating_sub, feature);
        simd_saturating_add(y, y);

        simd_saturating_add(z, z);
        //~^ ERROR expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type
        i32x4(0, 0, 0, 0);
        //~^ ERROR expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type
    }
}
