#![feature(cfg_target_has_atomic, no_core, intrinsics, lang_items)]
#![crate_type="rlib"]
#![no_core]

extern "rust-intrinsic" {
    fn atomic_xadd<T>(dst: *mut T, src: T) -> T;
}

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

#[cfg(target_has_atomic = "8")]
pub unsafe fn atomic_u8(x: *mut u8) {
    atomic_xadd(x, 1);
    atomic_xadd(x, 1);
}
