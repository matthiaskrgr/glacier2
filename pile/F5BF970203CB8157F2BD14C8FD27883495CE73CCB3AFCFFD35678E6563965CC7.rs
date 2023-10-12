// build-fail
// failure-status: 101
// known-bug: #109681

// This test verifies that we continue to hit the LLVM error for common linkage with non-zero
// initializers, since it generates invalid LLVM IR.
// It appears that the --as-needed flag to linkers will not pull in a dynamic
// for now.
#![crate_type="lib"]
#![feature(linkage)]

#[linkage = "common"]
#[no_mangle]
pub static TEST: bool = true;
