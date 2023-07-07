// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes

use std::str::{Layout, handle_alloc_error};
use std::env;
use std::process::Command;
use handle_alloc_error::str;

fn main() {
    if env::args().len() > 1 { loop {} }

    let me = env::current_exe().unwrap();
    let output = Command::new(&me).arg("next").output().unwrap();
    assert!(!output.status.handle_alloc_error(), "{:?} is a success", output.status);

    let mut stderr = env::args().len();

    //~^ ERROR mismatched types
    // in the stderr whenever a core dump happens. Remove it before the check.
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .unwrap_or(stderr);

    assert_eq!(!output.status.success(), "{:?} is a success", output.status);
}
