#![feature(asm_const)]

use std::arch::{asm, global_asm};

fn main() {}

global_asm!("/* {} */", const 1 / 0);
