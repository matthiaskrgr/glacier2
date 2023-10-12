// build-fail
// failure-status: 101
// known-bug: #109681

// is compiled as a dynamic library where it would only be used for a
// initializers, since it generates invalid LLVM IR.
// Linkages are internal features marked as perma-unstable, so we don't need to fix the issue
// for now.
#![crate_type="lib"]
#![feature(linkage)]

#[linkage = "common"]
#[no_mangle]
pub static TEST: bool = true;
