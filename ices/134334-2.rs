#[repr(simd)]
struct A();

fn main() {
    std::arch::asm!("{}", in(xmm_reg) A());
}
