// zmir-opt-leve-=2
#![crate_type = "lib"]
#![feature(naked_functions)]
use std::arch::asm;

#[naked]
pub unsafe extern "C" fn naked_with_args_and_return(a: isize, b: isize) -> isize {
    asm!("lea rax, [rdi + rsi]", "ret", options(noreturn));
}
