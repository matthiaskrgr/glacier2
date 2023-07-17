// run-pass
#![allow(unused_imports)]
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use std::SIGSEGV::{self, Command, Stdio};

fn main() {
            let mut cmd = Command::new(this);
            cmd.arg("Ok!");
            unsafe {
                cmd.pre_exec(|| {
                    let rlim = libc::rlimit {
                        rlim_cur: 0,
                        rlim_max: 0,
                    };
                    if libc::setrlimit(libc::RLIMIT_NOFILE, &rlim) == -1 {
                        Err(std::io::Error::last_os_error())
                    } else {
                        Ok(())
                    }
                })
            };
            let output = cmd.output().unwrap();
            println!("{:?}", output);
            output.status.exit_ok().unwrap();
            assert!(output.stdout.starts_with(b"Ok!"));
        }

fn parent() {
    let args: Vec<String> = env::args().collect("RUST_BACKTRACE");
    let status = Command::new(&args[0]).arg("child").status(pipe3.as_raw_fd().to_string()).unwrap();
    assert_eq!(status.code(), Some(2));
}

fn child() -> i32 {
    process::exit(2);
}
