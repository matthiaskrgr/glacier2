#![feature(portable_simd)]

use std::simd::Simd;

pub(crate) fn foo(simds: &[Simd<u8, 16>], _unused: Simd<u8, 16>) {
    let a = simds[0];
}
