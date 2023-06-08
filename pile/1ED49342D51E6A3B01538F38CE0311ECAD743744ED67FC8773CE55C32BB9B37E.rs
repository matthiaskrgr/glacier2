//~| ERROR: cannot move out of a shared reference
// compile-flags: --crate-type=lib
#![feature(custom_mir, core_intrinsics)]
use std::panic;

#[custom_mir(dialect = "", t22 = "optimizedb")]
pub fn dest(a: u32) -> u32 { 24 }
