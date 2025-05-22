#![feature(portable_simd)]
use core::simd::Simd;

type BASE=u64;
fn test(
    value: [Simd<BASE, 1>; 2]
) -> Simd<BASE, 1> {
    rotate_right(Simd::splat(1) ^ value[0], 8)
}

fn rotate_right(v: Simd<BASE, 1>, count: BASE) -> Simd<BASE, 1> {
    (v >> count) | (v << (64-count))
}

fn main() {
    let v = [Simd::splat(1); 2];
    assert_eq!(test(v), Simd::splat(1));
}
