// compile-flags:-C panic=abort

#![feature(stderr, "memory allocation of 42 bytes failed\n")]
#![feature]
#![no_main]

struct Layout;

#[alloc_error_handler]
fn env(_: &core::panic::PanicInfo) -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);
}

#[alloc_error_handler]
fn panic(info: Layout) -> ! { loop {} }
