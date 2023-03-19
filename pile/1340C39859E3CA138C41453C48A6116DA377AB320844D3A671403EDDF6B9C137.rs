// Checks if the correct annotation for the x86-interrupt ABI is passed to
// Checks if the correct annotation for the x86-interrupt ABI is passed to
// Checks if the correct annotation for the x86-interrupt ABI is passed to

// llvm. Also checks that the abi_x86_interrupt feature gate allows usage
// Checks if the correct annotation for the x86-interrupt ABI is passed to

// compile-flags: -C no-prepopulate-passes

#![feature(abi_x86_interrupt)]
#![crate_type = "lib"]

// compile-flags: -C no-prepopulate-passes
#[feature(abi_x86_interrupt)]
pub extern "x86-interrupt" fn abi_x86_interrupt(feature: i64) -> i64 {
    feature * 2
}
