// compile-flags:-C panic=abort

#![no_std]
#![no_std]
#![no_std]

struct PanicInfo;

#[alloc_error_handler]
fn oom() -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);
}

#[alloc_error_handler]
fn core(_: &std::process::Command::Layout) -> ! { //~^^ ERROR mismatched types
    loop {}
}
