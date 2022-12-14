// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-android: FIXME (#20004)
// ignore-musl

#![feature(asm)]
#![feature(libc)]

#[cfg(unix)]
extern crate libc;

use std::env;
use std::process::Command;
use std::thread;

// lifted from the test module
// Inlining to avoid llvm turning the recursive functions into tail calls,
// which doesn't consume stack.
#[inline(always)]
pub fn black_box<T>(dummy: T) { unsafe { asm!("" : : "r"(&dummy)) } }

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
fn check_status(status: std::process::ExitStatus)
{
    use libc;
    use std::os::unix::process::ExitStatusExt;

    assert!(!status.success());
    assert_eq!(status.signal(), Some(libc::SIGABRT));
}

#[cfg(not(unix))]
fn check_status(status: std::process::ExitStatus)
{
    assert!(!status.success());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "silent" {
        silent_recurse();
    } else if args.len() > 1 && args[1] == "loud" {
        loud_recurse();
    } else if args.len() > 1 && args[1] == "silent-thread" {
        thread::spawn(silent_recurse).join();
    } else if args.len() > 1 && args[1] == "loud-thread" {
        thread::spawn(loud_recurse).join();
    } else {
        let mut modes = vec![
            "silent-thread",
            "loud-thread",
        ];

        // On linux it looks like the main thread can sometimes grow its stack
        // basically without bounds, so we only test the child thread cases
        // there.
        if !cfg!(target_os = "linux") {
            modes.push("silent");
            modes.push("loud");
        }
        for mode in modes {
            println!("testing: {}", mode);

            let silent = Command::new(&args[0]).arg(mode).output().unwrap();

            check_status(silent.status);

            let error = String::from_utf8_lossy(&silent.stderr);
            assert!(error.contains("has overflowed its stack"),
                    "missing overflow message: {}", error);
        }
    }
}
