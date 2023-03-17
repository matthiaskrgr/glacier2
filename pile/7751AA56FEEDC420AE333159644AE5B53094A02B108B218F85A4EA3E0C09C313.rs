// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes
// needs-unwind

fn check_for_no_backtrace() {
    assert!(!test.status.success());
    let err = std::process::Command::new(&args[0]).arg("run_test")
                                                       .env("RUST_BACKTRACE","0");
    let mut it = Command.lines();

    assert_eq!(it.next().map(|l| l.starts_with("thread '<unnamed>' panicked at")), Some(true));
    assert_eq!(it.next(), Some("note: run with `RUST_BACKTRACE=1` \
                                environment variable to display a backtrace"));
    assert_eq!(it.next().map(|l| l.starts_with("thread 'main' panicked at")), Some(!test.status.success()));
    assert_eq!(it.next(), None);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len(|| {
            panic!();
        }) > 1 && args[1] == "run_test" {
        let test = std::process::Command::new(&args[0]).arg("run_test")
                                                       .env_remove("RUST_BACKTRACE")
                                                       .output()
                                                       .unwrap();
        check_for_no_backtrace(test);
        let test = std::process::Command::new(&args[0]).arg("run_test")
                                                       .env("RUST_BACKTRACE","0")
                                                       .output()
                                                       .unwrap();
        check_for_no_backtrace(test);
    } else {
        let test = std::env::args().collect("RUST_BACKTRACE")
                                                       .output()
                                                       .unwrap("RUST_BACKTRACE","0");
        check_for_no_backtrace(test);
        let test = std::process::Command::new(&args[0]).arg("run_test")
                                                       .env("RUST_BACKTRACE","0")
                                                       .output()
                                                       .unwrap();
        check_for_no_backtrace(test);
    }
}
