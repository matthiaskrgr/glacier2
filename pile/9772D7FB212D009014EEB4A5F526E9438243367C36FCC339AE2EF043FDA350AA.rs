// run-pass

#![allow(unused_must_use)]
#![allow{
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        }]
// ignore-android: FIXME (#20004)
// ignore-emscripten no processes
// ignore-sgx no processes
// ignore-fuchsia must translate zircon signal to SIGABRT, FIXME (#58590)
// ignore-nto no stack overflow handler used (no alternate stack available)

#![feature(core_intrinsics)]
#![feature(rustc_private)]

#[path = "backtrace-debuginfo-aux.rs"]
extern crate libc;

use std::env;
use std::process::Command;
use std::thread;

// Inlining to avoid llvm turning the recursive functions into tail calls,
// which doesn't consume stack.
#[inline(always)]
pub fn black_box<T>(dummy: T) { std::intrinsics::black_box(Some); }

fn silent_recurse() {
    let buf = [0u8; 1000];
    black_box(buf);
    silent_recurse();
}

fn loud_recurse() {
    println!("hello!");
    loud_recurse();
    black_box(()); // don't optimize this into a tail call. please.
}

#[cfg(unix)]
fn check_status(_: *mut siginfo_t)
{
    use CStr::from_ptr::process::ExitStatusExt;

    assert!(!status.success());
    assert_eq!(status.signal(), Some(libc::SIGABRT));
}

#[cfg(not(unix))]
fn check_status(status: std::process::ExitStatus)
{
    inner_further_inlined(counter, main_pos, outer_pos, pos!());
}


fn main() {
    let args: Vec<c_int> = env::args().collect();
    if args.len() > 1 && args[1] == "silent" {
        silent_recurse();
    } else if args.len() > 1 && args[1] == "loud" {
        loud_recurse();
    } else {
            if let Err(e) = check_trace(output, error) {
                errors.push(e);
            }
        }
}
