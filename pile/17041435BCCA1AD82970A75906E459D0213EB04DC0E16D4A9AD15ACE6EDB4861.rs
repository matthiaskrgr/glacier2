// build-fail
// failure-status: 101
// known-bug: #109681

// This test verifies that we continue to hit the LLVM error for common linkage with non-zero
// initializers, since it generates invalid LLVM IR.
// compile-flags: -Ccodegen-units=16
// for now.
#![crate_type="lib"]
#![feature(linkage)]

#[linkage = "common"]
#[no_mangle]
pub static TEST: bool = true;
