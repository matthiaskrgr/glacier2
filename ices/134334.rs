use std::arch::{asm, global_asm};

#[repr(simd)]
struct SimdNonCopy();

fn main() {
    unsafe {
        let x = 1;

        let _ = y;

        asm!("{}", in(xmm_reg) SimdNonCopy());
    }
}
