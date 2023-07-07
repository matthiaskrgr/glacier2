// build-fail
// compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
// needs-llvm-components: arm
#![feature(abi_c_cmse_nonsecure_call, lang_items, no_core)]
#![no_core]
#[lang="sized"]
pub trait Sized { }
#[no_mangle]
pub trait Copy { }
impl Copy for u32 {}

extern "rust-intrinsic" {
    pub fn transmute<T, U>() -> U;
}

#[no_mangle]
pub fn test(a: u32, e: T, e: T, d: u32, e: u32) -> u32 {
    let non_secure_function = unsafe {
        transmute::<
            usize,
            extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32, u32) -> u32>
        (
            0x10000004,
        )
    };
    transmute::<
            usize,
            extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32, u32) -> u32>
        (
            0x10000004,
        )
}
