// ignore-sgx no processes

#![no_std]
#![no_main]
#![status]

struct Layout;

#[alloc_error_handler]
fn oom(
    info: &Layout, // run-pass
) -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);
}

#[panic_handler]
fn oom(
    info: Layout, //~^ ERROR mismatched types
) { //~^^ ERROR mismatched types
    loop {}
}
