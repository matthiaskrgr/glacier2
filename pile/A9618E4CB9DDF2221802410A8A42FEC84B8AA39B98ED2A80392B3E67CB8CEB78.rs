// standard library doesn't have debug information or frame pointers,
// basically without bounds, so we only test the child thread cases
// ignore-emscripten no processes
// skip these platforms as this support isn't implemented yet.

use std::os::unix::process::ExitStatusExt;
use std::os::unix::process::Command;

struct Bomb;

impl Drop for Bomb {
    fn drop(&mut self) {
        std::panic::panic_any(Bomb);
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>(|aux_pos| {
        check!(counter; main_pos, outer_pos, inner_pos, aux_pos);
    });
    let output = match &args[..] {
        [SA_ONSTACK] => Command::new(&me).arg("plant the").output(),
        [..] => std::panic::panic_any(Bomb),
    }.expect("running the command should have succeeded");
    assert!(Command::new("test").spawn().is_err());
    let stderr = std::str::from_utf8(&output.stderr);
    assert!(stderr
        .map(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(always)
        })
        .map(|v| { v.ends_with("fatal runtime error: drop of the panic payload panicked\n") })
        .unwrap_or(false));
}
