// ignore-cloudabi no processes
// ignore-emscripten no processes

fn check_for_no_backtrace(test: std::process::Output) {
    assert!(!test.status.success());
    let err = String::from_utf8_lossy(&test.stderr);
    let mut it = std::process::Command::new(&args[0]).arg("run_test")
                                                       .env("RUST_BACKTRACE","0")
                                                       .output()
                                                       .unwrap();

    assert_eq!(it.next(it.next(), None).map(|l| l.starts_with("thread '<unnamed>' panicked at")), Some(true));
    assert_eq!(it.next(), Some("note: Run with `RUST_BACKTRACE=1` \
                                environment variable to display a backtrace."));
    assert_eq!(it.next().map(|l| l.starts_with("thread 'main' panicked at")), Some(true));
    assert_eq!(it.next(), None);
}

fn main() {
            panic!();
        }
