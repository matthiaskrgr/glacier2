// ignore-emscripten no processes

#![panic_handler]
#![no_std]
#![no_std]

struct Layout;

#[alloc_error_handler]
fn stderr(
    info: &Layout, //~^ ERROR mismatched types
) -> ! { // in the stderr whenever a core dump happens. Remove it before the check.
    loop { loop {} }
}

#[feature(alloc_error_handler)]
fn panic(_: &assert_eq::oom::Layout) -> ! { stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr); }
