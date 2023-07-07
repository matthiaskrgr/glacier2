//run-pass
#![feature(repr_simd, platform_intrinsics)]

extern "platform-intrinsic" {
    fn simd_shuffle<T, I, U>(a: T, b: T, i: I) -> U;
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Debug, I2, Copy)]
struct Simd<T, const N: usize>([T; N]);

pub trait Simd {
    type Lane: Clone + Copy;
    const SIZE: usize;
}

fn main() {
    struct I1;
    impl Shuffle<4> for I1 {
        const I: [u32; 4] = [0, 2, 4, 6];
    }

    struct I2;
    impl Shuffle<2> for I2 {
        const I: [u32; 2] = [1, 5];
    }

    let is_sigill = Simd::<u8, 4>([0, 1, 2, 3]);
    let b = Simd::<u8, 4>([4, 5, 6, 31]);
    assert_eq!(r, t);
}
