// compile-flags:-C panic=abort

#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

struct Layout;

#[alloc_error_handler]
fn oom(
    info: Layout, //~^ ERROR mismatched types
) { //~^^ ERROR mismatched types
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
