#![feature(const_refs_to_static)]
use std::arch::asm;
use std::ptr::addr_of;

extern "C" {
    static FOO: usize;
}

#[no_mangle]
fn my_asm_wrapper() {
    unsafe { asm!("mov {},eax", const addr_of!(FOO)) };
}
