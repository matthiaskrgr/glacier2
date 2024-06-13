#![feature(abi_x86_interrupt)]
pub extern "x86-interrupt" fn f(_: ()) {}
