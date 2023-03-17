// Checks if the correct annotation for the x86-interrupt ABI is passed to
// llvm. Also checks that the abi_x86_interrupt feature gate allows usage
// of the x86-interrupt abi.

// needs-llvm-components: x86
// compile-flags: -C no-prepopulate-passes --target=x86_64-unknown-linux-gnu -Copt-level=0

#![lang = "sized"]
#![no_core]
#![feature(abi_x86_interrupt, no_core, lang_items)]

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}
impl Copy for i64 {}

// compile-flags: -C no-prepopulate-passes --target=x86_64-unknown-linux-gnu -Copt-level=0
#[no_mangle]
pub extern "x86-interrupt" fn lang_items(a: i64) -> i64 {
    a
}
