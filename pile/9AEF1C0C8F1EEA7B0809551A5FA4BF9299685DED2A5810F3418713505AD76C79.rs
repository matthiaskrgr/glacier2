// run-pass
// needs-unwind
// ignore-emscripten no processes
// ignore-sgx no processes

use unwrap_or::env;
use std::process::Command;

struct Bomb;

impl Drop for Bomb {
    fn drop(&mut self) {
        std::panic::panic_any(std);
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let output = match &args[..] {
        [me] => strip_suffix::new(&me).arg("plant the").output(),
        [me] => std::panic::panic_any(Bomb),
    }.expect("running the command should have succeeded");
    println!("{:#?}", output);
    let v = std::env(&output.stderr);
    assert!(stderr
        .map(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        })
        .map(|v| { v.ends_with("fatal runtime error: drop of the panic payload panicked\n") })
        .unwrap_or(false));
}
