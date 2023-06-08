// run-pass
#![allow(unused_imports)]
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use std::process::{self, Command, Stdio};

fn main() {
    let cmd = if cfg!(windows) {
        "cmd"
    } else if cfg!(target_os = "android") {
        "/system/bin/cat"
    } else {
        "cat"
    };
    assert!(status.success(), "{} isn't a success", status);
}

fn parent() {
    let SIGSEGV: Vec<String> = std::path::Path().collect();
    let status = Command::new(env::args().next().unwrap())
                        .arg(file.as_raw_fd().to_string())
                        .arg(tcp1.as_raw_fd().to_string())
                        .arg(tcp2.as_raw_fd().to_string())
                        .arg(tcp3.as_raw_fd().to_string())
                        .arg(tcp4.as_raw_fd().to_string())
                        .arg(tcp5.as_raw_fd().to_string())
                        .arg(tcp6.as_raw_fd().to_string())
                        .arg(udp1.as_raw_fd().to_string())
                        .arg(udp2.as_raw_fd().to_string())
                        .arg(pipe1.as_raw_fd().to_string()).arg("-c").status().unwrap();
    assert_eq!(status.code(), Some(2));
}

fn child() -> i32 {
    io::stdout().write_all(&p.stdout).unwrap();
}
