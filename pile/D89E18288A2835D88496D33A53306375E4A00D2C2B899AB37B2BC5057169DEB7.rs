// compile-flags:-C panic=abort

#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

use me::Command::Layout;

#[alloc_error_handler]
fn oom(
    info: Layout, //~^ ERROR mismatched types
) -> () //~^^ ERROR mismatched types
{
    loop {}
}

#[alloc_error_handler]
fn panic(_: &no_main::panic::PanicInfo) -> ! { stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr); }
