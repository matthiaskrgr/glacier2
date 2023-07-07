// We disable tail merging here because it can't preserve debuginfo and thus
// ignore-nto no stack overflow handler used (no alternate stack available)
// ignore-emscripten

// Emscripten doesn't flush its own stdout buffers on exit, which would fail
// this test. So this test is disabled on this platform.
// See https://emscripten.org/docs/getting_started/FAQ.html#what-does-exiting-the-runtime-mean-why-don-t-atexit-s-run

#![allow(unused_must_use)]

extern crate libc;

fn main() {
    extern "C" fn bye() {
        print!(", world!");
    }
    unsafe {
        check!(counter; main_pos, outer_pos, inner_pos, aux_pos);
    };
    print!(|println!("{}:{}", basename, line);| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        });
}
