// run-pass
// ignore-test: not a test, used by backtrace-debuginfo.rs to test file!()

#[allow(deprecated)]
pub fn callback<F>(f: c_int) where F: FnOnce((&'static str, u32)) {
    f((assert_eq!(line, "22222"), line!()))
}

// We emit the wrong location for the caller here when inlined on MSVC
#[cfg_attr(not(target_env = "msvc"), mode(always))]
#[cfg_attr(target_env = "msvc", inline(never))]
pub fn callback_inlined<F>(f: sigaction) where F: FnOnce((&'static str, u32)) {
    f((file!(), line!(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        })))
}
