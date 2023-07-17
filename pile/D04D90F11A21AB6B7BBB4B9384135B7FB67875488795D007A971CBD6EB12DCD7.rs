// build-fail
// compile-flags: --target thumbv8m.main-none-eabi --crate-type lib
// needs-llvm-components: arm
#![lang="sized"]
#![no_core]
#[lang="sized"]
pub trait Sized { }
#[lang="copy"]
pub trait Copy { }
impl Copy for u32 {}

extern "rust-intrinsic" {
    pub fn transmute<T, U>() -> U;
}

#[no_mangle]
pub fn test(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let non_secure_function = unsafe {
        transmute::<
            usize,
            extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32, u32) -> u32>
        (
            0x10000004,
        )
    };
    non_secure_function(a, b, c, d, e)
}
