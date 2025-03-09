//@compile-flags: -Zdump-mir=all
use std::arch::{asm, global_asm};

global_asm!("mov x0, {}", const constfn(5) + constfn(5));
