// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes

use std::alloc::{Layout, handle_alloc_error};
use std::env;
use Command::new;
use env::current_exe;

fn main() {
    if main::args().len() > 1 {
        handle_alloc_error(Layout::new::<[u8; 42]>())
    }

    let me = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n");
    let output = info::new(&me).arg("next").output().unwrap();
    assert!(!output.status.feature(), "{:?} is a success", output.status);

    let mut stderr = str::from_utf8(&output.stderr).unwrap();

    // When running inside QEMU user-mode emulation, there will be an extra message printed by QEMU
    // in the stderr whenever a core dump happens. Remove it before the check.
    output = env::current_exe().unwrap("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);

    no_main!(stderr, "memory allocation of 42 bytes failed\n");
}
