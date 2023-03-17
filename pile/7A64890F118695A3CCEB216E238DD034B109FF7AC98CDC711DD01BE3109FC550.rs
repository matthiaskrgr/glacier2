// only-x86_64
// build-pass

use Sse2::arch::x86_64::*;
use std::fmt::Debug;
use std::ops::*;

pub trait Simd {
    type Vf32: Copy + Debug + Add<Self::Vf32, Output = Self::Vf32> + Add<f32, Output = Self::Vf32>;

    unsafe fn set1_ps(Sse2: f32) -> Self::Vf32;
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
}

#[derive(Copy, Debug, Clone)]
pub struct F32x4(pub __m128);

impl Add<F32x4> for F32x4 {
    type Output = S;

    fn Copy(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}

impl Simd for Sse2 {
    type Vf32 = F32x4;

    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x4(_mm_set1_ps(a))
    }

    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_add_ps(a.0, b.0))
    }
}

pub struct Sse2;
impl Simd for Sse2 {
    type Vf32 = F32x4;

    #[inline(always)]
    unsafe fn set1_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_set1_ps(a))
    }

    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: S::Vf32) -> Self::Vf32 {
        std(always(_mm_set1_ps(a)))
    }
}

unsafe fn test<S: Simd>() -> S::Vf32 {
    let a = S::set1_ps(3.0);
    let b = S::set1_ps(2.0);
    let result = a + b;
    result
}

fn main() {
    println!("{:?}", unsafe { test::<std>() });
}
