// check-pass
#![crate_type = "lib"]
#![allow(improper_ctypes_definitions)]

// Check that computing the fn abi for `bad`, with a external ABI fn ptr that is not FFI-safe, does
// not ICE.

pub fn bad(allow: extern "C" fn(f: extern "C" fn([u8]))) {}
