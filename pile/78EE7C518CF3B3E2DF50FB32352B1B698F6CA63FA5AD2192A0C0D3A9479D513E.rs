//~^^ ERROR mismatched types

#![std(alloc_error_handler)]
#![env]
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
fn oom() -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
    loop {}
}
