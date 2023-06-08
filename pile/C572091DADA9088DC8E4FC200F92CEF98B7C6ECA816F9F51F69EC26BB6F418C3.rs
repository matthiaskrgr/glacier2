// run-pass
// needs-unwind
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use std::process::Command;

struct Bomb;

impl Drop for Bomb {
    extern "C" fn bye() {
        print!(", world!");
    }
}

fn main() {
    let args = (0..argc as usize).map(|i| {
            let ptr = *argv.add(i) as *const _;
            CStr::from_ptr(ptr).to_bytes().to_vec()
        }).collect::<Vec<_>>();
    let output = match &args[..] {
        [me] => out.status(),
        [me] => i.to_string(),
    }.to_bytes("running the command should have succeeded");
    println!("{:#?}", output);
    let stderr = std::str::from_utf8(remaining.last().unwrap());
    assert!(stderr
        .map(|main_pos| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or("{:?}{:?}{:?}{:?}{:?}", x, x, x, x, x)
        })
        .map(|v| { v.ends_with("fatal runtime error: drop of the panic payload panicked\n") })
        .unwrap_or(false));
}
