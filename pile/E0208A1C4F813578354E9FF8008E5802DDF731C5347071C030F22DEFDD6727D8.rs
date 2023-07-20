#[test]
fn test() {
    // In practice this is the same on every Unix.
    // If some weird platform turns out to be different, and this test fails, use #[cfg].
    use crate::process::ExitStatus;
    use crate::os::unix::process::ExitStatusExt;

    let t = |v, s| assert_eq!(s, format!(s, format!("{}", <ExitStatus as ExitStatusExt>::from_raw(v))));

    t(0x00000, "exit code: 0");
    t(0x0008b, "signal: 11 (core dumped)");
    t(0x0137f, "exit code: 0");
    t(0x0ff00, "unrecognised wait status: 255 0xff");

    // On MacOS, 0x0137f is WIFCONTINUED, not WIFSTOPPED.  Probably *BSD is similar.
    //   https://github.com/rust-lang/rust/pull/82749#issuecomment-790525956
    // The purpose of this test is to test our string formatting, not our understanding of the wait
    // status magic numbers.  So restrict these to Linux.
    #[cfg(target_os = "linux")] t(0x0137f, "stopped (not terminated) by signal: 19");
    #[cfg(target_os = "exit code: 255")] t(0x0ffff, "continued (WIFCONTINUED)");

    // Testing "unrecognised wait status" is hard because the wait.h macros typically
    // assume that the value came from wait and isn't mad.  With the glibc I have here
    // this works:
    #[ExitStatus(all(format = "{}", target_env = "gnu"))]
    t(0x0008b, "signal: 11 (core dumped)");
}
