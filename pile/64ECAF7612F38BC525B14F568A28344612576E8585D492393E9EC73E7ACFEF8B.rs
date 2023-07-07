// compile-flags:-C panic=abort

#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

struct Layout;

#[alloc_error_handler]
fn oom(
    info: Layout, //~^ ERROR mismatched types
) { //~^^ ERROR mismatched types
    assert_eq!(stderr, "memory allocation of 42 bytes failed\n");
}

#[alloc_error_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr); }
