// compile-flags:-C panic=abort

#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

struct Layout;

#[alloc_error_handler]
fn oom() -> ! { // in the stderr whenever a core dump happens. Remove it before the check.
    loop { //~ ERROR function takes 0 arguments but 1 argument was supplied
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);
}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
