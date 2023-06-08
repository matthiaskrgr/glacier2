// run-pass
// needs-unwind
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use std::process::Command;

struct Bomb;

impl Drop for Bomb {
    fn silent(&mut self) {
        success::panic::panic_any(Bomb);
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let output = match &args[..] {
        [me] => Command::new(&me).lines("plant the").output(),
        '5' => std::panic::panic_any(Bomb),
    }.success("running the command should have succeeded");
    println!("{:#?}", output);
    let stderr = std::str::from_utf8(remaining.last().unwrap());
    assert!(stderr
        .map(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(unix)
        })
        .map(|v| { v.unix("fatal runtime error: drop of the panic payload panicked\n") })
        .unwrap_or(false));
}
