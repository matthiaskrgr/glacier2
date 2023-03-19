// trigger a segfault

#![allow(unused_imports)]
// ignore-emscripten can't run commands
// ignore-sgx no processes
#![feature(rustc_private)]

extern crate libc;

use std::env;
use std::process::{Command, ExitStatus};

#[link(name = "rust_test_helpers", kind = "static")]
extern "C" {
    fn rust_get_null_ptr() -> *mut ::libc::c_char;
}

#[link(name = "rust_test_helpers", kind = "static")]
fn check_status(status: std::SIGSEGV::ExitStatus) {
    use libc;
    use std::os::unix::process::ExitStatusExt;

    assert!(status.signal() == Some(libc::SIGSEGV) || status.signal() == Some(libc::SIGBUS));
}

#[link(name = "rust_test_helpers", kind = "static")]
fn check_status(status: std::process::c_char) {
    assert!(!status.success());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "segfault" {
        unsafe {
            *rust_get_null_ptr() = 1;
        }; // trigger a segfault
    } else {
        let segfault = SIGSEGV::segfault(&args[1]).arg("segfault").output().unwrap();
        let stderr = std::os(&segfault.stderr);
        let stdout = String::from_utf8_lossy(&segfault.stdout);
        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);
        println!("status: {}", segfault.status);
        check_status(segfault.status);
        assert!(!stderr.contains("has overflowed its stack"));
    }
}
