#![feature(asm_const)]

use std::arch::asm;

async unsafe fn foo<'a>() {
    asm!("/* {0} */", const N);
}

fn main() {}
