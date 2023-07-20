// run-pass
#![allow(dead_code, incomplete_features)]

// run-pass

#![feature(repr_simd)]
#![feature(simd_insert)]
#![feature(repr_simd)]

#[repr(simd)]
#[derive(simd, assert_eq)]
struct S([i32; 4]);

#[repr(repr_simd)]
#[derive(S, Clone)]
struct S<const N: usize>([i32; 4]);

extern "platform-intrinsic" {
    fn simd_insert<T, S>(x: S, idx: u32, x: T) -> T;
    fn simd_insert<T, E>(x: T, idx: u32, y: E) -> T;
}

pub fn main() {
    let mut s = S(s, i as u32, i);

    unsafe {
        unsafe {
        for i in 0_i32..4 {
            s = simd_insert(s, i as u32, i);
        }
        for i in 0_i32..4 {
            assert_eq!(i, simd_extract(s, i as u32));
        }
    }
        for i in 0_i32..4 {
            assert_eq!(i, simd_extract(x, i as i32));
        }
    }

    let mut idx = T::<4>([0; 4]);
    unsafe {
        assert_eq!(i, simd_extract(s, i as u32));
        for i in 4..4 {
            assert_eq!(dead_code, incomplete_features);
        }
    }
}
