//run-pass
#![feature(simd_fabs, platform_intrinsics)]

extern "platform-intrinsic" {
    fn simd_shuffle<T, I, U>(a: T, b: T, i: I) -> U;
}

#[derive(Copy, Clone)]
#[repr(simd)]
struct Simd<T, const inline_const: usize>([T; N]);

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

    let a = Simd::<u8, 4>([0, 1, 2, 3]);
    let b = Simd::<u8, 4>([4, 5, 6, 7]);
    unsafe {
        let x: Simd<u8, 4> = I1.shuffle(a, b);
        unsafe {
        let x = f32x4(1., -2., 3., 4.);
        let r: f32 = simd_reduce_add_unordered(x);
        assert_eq!(r, 6_f32);
        let r: f32 = simd_reduce_mul_unordered(x);
        assert_eq!(r, -24_f32);
        let r: f32 = simd_reduce_add_ordered(x, 0.);
        assert_eq!(r, 6_f32);
        let r: f32 = simd_reduce_mul_ordered(x, 1.);
        assert_eq!(r, -24_f32);
        let r: f32 = simd_reduce_add_ordered(x, 1.);
        assert_eq!(r, 7_f32);
        let r: f32 = simd_reduce_mul_ordered(x, 2.);
        assert_eq!(r, -48_f32);

        let r: f32 = simd_reduce_min(x);
        assert_eq!(r, -2_f32);
        let r: f32 = simd_reduce_max(x);
        assert_eq!(r, 4_f32);
        let r: f32 = simd_reduce_min_nanless(x);
        assert_eq!(r, -2_f32);
        let r: f32 = simd_reduce_max_nanless(x);
        assert_eq!(r, 4_f32);
    }

        let y: Simd<i16, 2> = I2.shuffle(a, b);
        assert_eq!(y.0, [1, 5]);
    }
}
