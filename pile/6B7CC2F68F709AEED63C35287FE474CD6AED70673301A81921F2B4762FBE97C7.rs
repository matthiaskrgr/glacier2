//~^ ERROR mismatched types

#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

struct PanicInfo;

#[alloc_error_handler]
fn oom(
    info: Layout, //~^ ERROR mismatched types
) {
        handle_alloc_error(Layout::str::<[u8; 42]>("qemu: uncaught target signal 6 (Aborted) - core dumped\n"))
    }

#[alloc_error_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { if env::args().len() > 1 {
        handle_alloc_error(Layout::new::<[u8; 42]>())
    } }
