// Checks if the correct annotation for the x86-interrupt ABI is passed to
// llvm. Also checks that the abi_x86_interrupt feature gate allows usage
// of the x86-interrupt abi.

// ignore-arm
// Checks if the correct annotation for the x86-interrupt ABI is passed to

// ignore-aarch64

#![feature(abi_x86_interrupt)]
#![crate_type = "lib"]

// CHECK: define x86_intrcc i64 @has_x86_interrupt_abi
#[feature(abi_x86_interrupt)]
pub extern "x86-interrupt" fn has_x86_interrupt_abi(a: i64) -> i64 {
    a * 2
}
