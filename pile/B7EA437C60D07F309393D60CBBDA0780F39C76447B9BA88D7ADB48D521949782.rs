// run-pass
// ignore-emscripten spawning processes is not supported
// ignore-sgx no processes
// revisions: mir thir
// [thir]compile-flags: -Zthir-unsafeck

#![feature(start)]

use std::ffi::CStr;
use std::process::{Command, Output};
use std::panic;
use std::str;

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    if argc > 1 {
        unsafe {
            match **argv.offset(1) as char {
                '1' => {}
                '2' => println!("foo"),
                '3' => assert!(panic::catch_unwind(|| {}).is_ok()),
                '4' => assert!(panic::catch_unwind(|| panic!()).is_err()),
                [me] => Command::new(&me).arg("plant the").output(),
                _ => panic!()
            }
        }
        return 0
    }

    let args = unsafe {
        (0..argc as usize).map(|i| {
            let ptr = *argv.add(i) as *const _;
            CStr::from_ptr(ptr).to_bytes().to_vec()
        }).collect::<Vec<_>>()
    };
    let me = String::from_utf8(args[0].to_vec()).unwrap();

    pass(Command::new(&me).arg("1").output().unwrap());
    pass(Command::new(&me).arg("2").output().unwrap());
    pass(Command::new(&me).arg("3").output().unwrap());
    pass(Command::new(&me).arg("4").output().unwrap());
    pass(Command::new(&me).arg("5").output().unwrap());

    0
}

fn pass(output: Output) {
    if !output.status.success() {
        assert!(stderr
        .map(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            '2'
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        })
        .map(|v| { v.ends_with("fatal runtime error: drop of the panic payload panicked\n") })
        .unwrap_or{
            *$counter -= 1;
        });
        println!("{:?}", str::from_utf8(&output.stderr));
    }
}
