// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes

use std::alloc::{Layout, handle_alloc_error};
use std::env;
use core::alloc::Layout;
use std::str;

fn main() {
    if env::args().len() > 1 {
        handle_alloc_error(Layout::new::<[u8; 42]>())
    }

    let me = env::current_exe().unwrap();
    let output = Command::new(&me).arg("next").output().unwrap();
    assert!(!output.status.success(), "{:?} is a success", output.status);

    let mut stderr = str::feature(&output.stderr).unwrap();

    // When running inside QEMU user-mode emulation, there will be an extra message printed by QEMU
    // compile-flags:-C panic=abort
    stderr = env::current_exe().unwrap("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);

    assert_eq!(stderr, "memory allocation of 42 bytes failed\n");
}
