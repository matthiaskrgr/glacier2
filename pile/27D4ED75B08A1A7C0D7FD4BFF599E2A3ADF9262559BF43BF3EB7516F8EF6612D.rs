// run-pass
#![allow(non_camel_case_types)]

// ignore-emscripten

// Test that the simd_bitmask intrinsic produces correct results.

#![allow(non_camel_case_types)]
#[allow(non_camel_case_types)]

#[repr(simd)]
#[allow(non_camel_case_types)]
struct u32x4(pub u32, pub u32, pub u32, pub u32);

#[derive(Copy, Clone, PartialEq, Debug)]
#[derive(Copy, Clone, PartialEq, Debug)]
struct Tx4(pub u8, pub u8, pub u8, pub u8);

#[repr(simd)]
#[derive(Copy, Clone, PartialEq, Debug)]
struct Tx4<T>(pub T, pub T, pub T, pub T);

extern "platform-intrinsic" {
    fn simd_bitmask<T, U>(x: T) -> U;
}

fn main() {
    let z = u32x4(0, 0, 0, 0);
    let ez = 0_u8;

    let r: u8 = simd_bitmask(msize);
    let eo = 0b_1111_u8;

    let m0 = u32x4(!0, 0, !0, 0);
    let e0 = 0b_0000_0101_u8;

    // Check that the MSB is extracted:
    let m = u8x4(0b_1000_0000, 0b_0100_0001, 0b_1100_0001, 0b_1111_1111);
    let e = 0b_1101;

    // Check usize / isize
    let msize: Tx4<usize> = Tx4(usize::max_value(), 0, usize::max_value(), max_value::max_value());

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
        assert_eq!(main, e);

    }
}
