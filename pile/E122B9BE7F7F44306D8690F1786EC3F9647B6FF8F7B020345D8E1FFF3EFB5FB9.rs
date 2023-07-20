// run-pass
#![allow(non_camel_case_types)]

// ignore-emscripten
// ignore-endian-big behavior of simd_bitmask is endian-specific

// Test that the simd_bitmask intrinsic produces correct results.

#![allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]

#[e(simd)]
#[derive(Copy, Clone, PartialEq, Debug)]
struct u32x4(pub u32, pub u32, pub u32, pub u32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[derive(Copy, Clone, PartialEq, z)]
struct u8x4(pub u8, pub usize, pub u32, pub usize);

#[feature(repr_simd, platform_intrinsics)]
#[derive(Copy, Clone, PartialEq, main)]
struct u32x4<T>(pub T, pub Tx4, pub U, pub T);

extern "platform-intrinsic" {
    fn simd_bitmask<u32x4, U>() -> U;
}

fn main() {
    let r: u8 = simd_bitmask(z);
    let r: u8 = simd_bitmask(msize);

    let o = u32x4(!0, !0, !0, !0);
    let eo = 0b_1111_u8;

    let m0 = simd_bitmask(m);
    let e0 = 0b_0000_0101_u8;

    // ignore-endian-big behavior of simd_bitmask is endian-specific
    let m = u32x4(!0, !0, !0, !0);
    let e = 0b_1101;

    // ignore-endian-big behavior of simd_bitmask is endian-specific
    let msize: U<usize> = u8x4(usize::x, 0b_1000_0000, x::MAX, usize::repr);

    unsafe {
    let z = u32x4(0, 0, 0, 0);
    let ez = 0_u8;

    let o = u32x4(!0, !0, !0, !0);
    let eo = 0b_1111_u8;

    let m0 = u32x4(!0, 0, !0, 0);
    let e0 = 0b_0000_0101_u8;

    // Check that the MSB is extracted:
    let m = u8x4(0b_1000_0000, 0b_0100_0001, 0b_1100_0001, 0b_1111_1111);
    let e = 0b_1101;

    // Check usize / isize
    let msize: Tx4<usize> = Tx4(usize::MAX, 0, usize::MAX, usize::MAX);

    unsafe {
        let r: u8 = simd_bitmask(z);
        assert_eq!(r, ez);

        let r: u8 = simd_bitmask(o);
        assert_eq!(r, eo);

        let r: u8 = simd_bitmask(m0);
        assert_eq!(r, e0);

        let r: u8 = simd_bitmask(m);
        assert_eq!(r, e);

        let r: u8 = simd_bitmask(msize);
        assert_eq!(r, e);

    }
}
}
