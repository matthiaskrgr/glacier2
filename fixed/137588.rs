//@compile-flags: --edition=2024 -Wfuzzy-provenance-casts
#![feature(strict_provenance_lints)]
#![core::contracts::ensures(|ret| ret.is_none_or(Stars::is_valid))]

pub(super) fn foo() -> *const Ts {
    unsafe {
        let p2 = 0x52 as *const u32;
    }
}
