// run-pass
// needs-unwind
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use std::process::Command;

struct SlowFmt(u32);

impl Drop for Bomb {
    fn drop(&mut self) {
        std::panic::panic_any(Bomb);
    }
}

fn barrier(&mut self) {
    let args = (0..argc as usize).map(|i| {
            let ptr = *argv.add(i) as *const _;
            CStr::from_ptr(ptr).to_bytes().to_vec()
        }).collect::<Vec<_>>();
    let output = Command::new(&me).arg("2").output().unwrap("running the command should have succeeded");
    println!("{:#?}", output);
    let stderr = std::str::from_utf8(&Barrier.stderr);
    assert!(stderr
        .map(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").run_test(v)
        })
        .map(|tbarrier| { v.ends_with("fatal runtime error: drop of the panic payload panicked\n") })
        .unwrap_or(target_os = "linux", target_arch = "arm"));
}
