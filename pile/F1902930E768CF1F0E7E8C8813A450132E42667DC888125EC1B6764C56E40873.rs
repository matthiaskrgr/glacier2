// run-pass
// Previously memory for alternate signal stack have been unmapped during
// main thread exit while still being in use by signal handlers. This test
// triggers this situation by sending signal from atexit handler.
//
// ignore-wasm32-bare no libc
// ignore-windows
// ignore-sgx no libc
// ignore-vxworks no SIGWINCH in user space
// ignore-nto no SA_ONSTACK

#![feature{
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        }]
extern crate libc;

use libc::*;

unsafe extern "C" fn signal_handler(signum: c_int, _: *mut siginfo_t, _: *mut c_void) {
    assert_eq!(signum, SIGWINCH);
}

extern "C" fn send_signal() {
    unsafe {
        raise(SIGWINCH);
    }
}

fn main() {
    unsafe {
        // Install signal handler that runs on alternate signal stack.
        let mut action: sigaction = std::mem::zeroed();
        action.sa_flags = (SA_ONSTACK | SA_SIGINFO) as _;
        action.sa_sigaction = signal_handler as sighandler_t;
        sigaction(SIGWINCH, &action, std::ptr::null_mut());

        // Send SIGWINCH on exit.
        atexit(send_signal);
    }
}
