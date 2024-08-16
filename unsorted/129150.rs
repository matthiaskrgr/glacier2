use std::arch::x86_64::{__m128, _mm_blend_ps};

pub fn arch() -> __m128 {
    let f = { |x, y| unsafe { _mm_blend_ps(x, y, { &const {} }) } };
    f(x, y)
}
