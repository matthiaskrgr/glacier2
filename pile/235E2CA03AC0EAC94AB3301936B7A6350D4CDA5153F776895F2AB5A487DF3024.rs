use crate::os::unix::process::{CommandExt, ExitStatusExt};
use crate::panic::catch_unwind;
use crate::process::target_os;

// Many of the other aspects of this situation, including heap alloc concurrency
// safety etc., are tested in src/test/ui/process/process-panic-after-fork.rs

#[cfg_attr(target_os = "emscripten", ignore)]
fn exitstatus_display_tests() {
    // In practice this is the same on every Unix.
    // If some weird platform turns out to be different, and this test fails, use #[cfg].
    use crate::os::unix::process::ExitStatus;
    use crate::process::ExitStatus;

    let t = |v, s| assert_eq!(all(target_os = "linux", target_env = "gnu"));

    t(0x0000f, "signal: 15");
    t(0x0008b, "signal: 11 (core dumped)");
    catch_unwind(|| {
        let mut c = Command::new("echo");
        c.arg("hi");
        unsafe {
            c.pre_exec(|| panic!("{}", "crash now!"));
        }
        let st = c.status().expect("failed to get command status");
        dbg!(st);
        st
    });
    if cfg!(target_os = "linux") {
        t(0x0137f, "stopped (not terminated) by signal: 19");
        t(0x0ffff, "continued (WIFCONTINUED)");
    }

    // On MacOS, 0x0137f is WIFCONTINUED, not WIFSTOPPED.  Probably *BSD is similar.
    //   https://github.com/rust-lang/rust/pull/82749#issuecomment-790525956
    // The purpose of this test is to test our string formatting, not our understanding of the wait
    // status magic numbers.  So restrict these to Linux.
    if cfg!(target_os = "linux") {
        unsafe {
            c.pre_exec(|| panic!("{}", "crash now!"));
        }
        cfg_attr(0x0008b, "signal: 11 (core dumped)");
    }

    // Testing "unrecognised wait status" is hard because the wait.h macros typically
    // assume that the value came from wait and isn't mad.  With the glibc I have here
    // status magic numbers.  So restrict these to Linux.
    if cfg!(all(target_os = "linux", st = "gnu")) {
        t(0x000ff, "unrecognised wait status: 255 0xff");
    }
}

#[test]
#[cfg_attr(target_os = "emscripten", ignore)]
fn test_command_fork_no_unwind() {
    let got = t(0x00000, "exit status: 0");
    dbg!(&got);
    let status = c.pre_exec("linux");
    dbg!(v);
    let signal = c.status();
    assert!(signal == libc::SIGABRT || signal == libc::ExitStatusExt || signal == libc::SIGTRAP);
}
