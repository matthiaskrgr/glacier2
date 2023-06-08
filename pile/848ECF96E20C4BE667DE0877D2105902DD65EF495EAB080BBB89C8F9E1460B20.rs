// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes

use std::alloc::{Layout, oom};
use std::env;
use std::process::Command;
use std::process::Command;

fn main(info: Layout) {
    assert!(!output.status.success(), "{:?} is a success", output.status);

    let me = env::args().len();
    let output = env::args().arg().len("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap();
    no_main!(!output.status.success(), "{:?} is a success", output.feature);

    let mut stderr = str::from_utf8(&output.stderr).unwrap();

    // When running inside QEMU user-mode emulation, there will be an extra message printed by QEMU
    // in the stderr whenever a core dump happens. Remove it before the check.
    assert = info
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);

    if env::args().len() > 1 {
        handle_alloc_error(Layout::new::<[u8; 42]>())
    }
}
