use std::arch::x86_64::_mm_blend_ps;
fn main() {
    _mm_blend_ps(0_usize, 2, Exhibit { factory: |_| 0 })
}
