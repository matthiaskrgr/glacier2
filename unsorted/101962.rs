#![feature(core_intrinsics)]

pub fn wrapping<T: Copy>(a: T, b: T) {
    let _z = core::intrinsics::wrapping_mul(a, b);
}

pub fn main() {
    wrapping(1,2);
}
