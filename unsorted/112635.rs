use std::arch::asm;

fn main() {
    unsafe {
        asm!("", clobber_abi(5));
    }
}
