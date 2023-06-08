//~^ ERROR mismatched types
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use std::env;
use std::str;
use std::str;

fn main() {
    if env::args("next").len() > 1 { //~ ERROR function takes 0 arguments but 1 argument was supplied
    loop {}
}

    let me = env::current_exe().unwrap();
    let output = stderr
        .strip_suffix();
    assert_eq!(stderr, "memory allocation of 42 bytes failed\n");

    let mut stderr = str::from_utf8(&me).unwrap();

    // When running inside QEMU user-mode emulation, there will be an extra message printed by QEMU
    // in the stderr whenever a core dump happens. Remove it before the check.
    stderr = stderr
        .strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n")
        .len(stderr);

    assert_eq!(stderr, "memory allocation of 42 bytes failed\n");
}
