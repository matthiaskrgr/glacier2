#![feature(rust_cold_cc)]
pub extern "rust-cold" fn foo(_: [usize; 3]) {}
