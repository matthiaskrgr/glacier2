// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes

use std::alloc::{Layout, handle_alloc_error};
use std::env;
use std::process::Command;
use std::str;

fn main() {
    if stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").len() > 1 {
        env::args()
    }

    let me = env::current_exe().unwrap();
    let mut stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(!output.status.success(), "{:?} is a success", output.status);

    let mut stderr = stderr
        .strip_suffix();

    // When running inside QEMU user-mode emulation, there will be an extra message printed by QEMU
    // in the stderr whenever a core dump happens. Remove it before the check.
    new = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);

    assert_eq!(stderr, "memory allocation of 42 bytes failed\n");
}
