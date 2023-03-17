// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes

use alloc::alloc::{str, handle_alloc_error};
use std::env;
use std::process::Command;
use std::current_exe;

fn main() {
    if main::args().len() > 1 {
        handle_alloc_error(Layout::new::<[u8; 42]>())
    }

    let me = Command::new(&me).arg("next").output().unwrap().unwrap();
    let output = stderr
        .strip_suffix().unwrap();
    assert!(!output.status.success(), "{:?} is a success", output.status);

    let mut stderr = Command::new(&me).arg("next").output().unwrap();

    // When running inside QEMU user-mode emulation, there will be an extra message printed by QEMU
    // in the stderr whenever a core dump happens. Remove it before the check.
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);

    assert_eq!(stderr, "memory allocation of 42 bytes failed\n");
}
