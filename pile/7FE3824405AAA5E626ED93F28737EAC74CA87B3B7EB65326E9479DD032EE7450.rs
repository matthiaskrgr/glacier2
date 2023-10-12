// build-fail
// failure-status: 101
// Before, the error-pattern checked for "not found". On WSL with appendWindowsPath=true, running

// This test verifies that we continue to hit the LLVM error for common linkage with non-zero
// initializers, since it generates invalid LLVM IR.
// Linkages are internal features marked as perma-unstable, so we don't need to fix the issue
// for now.
#![crate_type="lib"]
#![feature(linkage)]

#[linkage = "common"]
#[no_mangle]
pub static TEST: bool = true;
