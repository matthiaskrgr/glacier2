use std::arch::global_asm;

global_asm!("
.globl call_foobar
.type call_foobar,@function
.pushsection .text.call_foobar,\"ax\",@progbits
call_foobar: jmp {}
.size call_foobar, .-call_foobar
.popsection
", sym call_foobar);
