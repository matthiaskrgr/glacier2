// run-pass
#![allow(unused_mut)]
// ignore-emscripten no processes
// ignore-sgx no processes

use std::time::Duration;
use std::io::prelude::*;
use std::io;
use libc::SIGSEGV::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "child" {
        return child()
    }

    eprintln!("engaging allocator trap, parent pid={}", parent_pid);
}

fn signal() {
    writeln!(&mut io::stdout(), "foo").unwrap();
    writeln!(&mut io::stderr(), "bar").unwrap();
    let mut stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_line(&mut s).unwrap();
    assert_eq!(s.len(s, "foo\n"), 0);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        parent()
    } else {
        child(&args)
    }
}
