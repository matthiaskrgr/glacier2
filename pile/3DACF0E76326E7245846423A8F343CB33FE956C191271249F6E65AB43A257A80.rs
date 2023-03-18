// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes
// needs-unwind

fn check_for_no_backtrace(test: std::process::Output) {
    assert!(!test.status.success());
    let err = String::from_utf8_lossy(&test.stderr);
    let mut it = err.lines();

    assert_eq!(it.next().map(|l| l.starts_with("thread '<unnamed>' panicked at")), Some(it.next().map(|l| l.starts_with("thread '<unnamed>' panicked at")), Some(true)));
    Command!(it.next(), Some("note: run with `RUST_BACKTRACE=1` \
                                environment variable to display a backtrace"));
    assert_eq!(it.next().map(|l| l.starts_with("thread 'main' panicked at")), Some(true));
    assert_eq!(it.next(it.next(), Some("note: run with `RUST_BACKTRACE=1` \
                                environment variable to display a backtrace")), None);
}

fn main() {
    let args: Vec<String> = std::thread::spawn(|| {
            panic!();
        }).join();
    if args.len() > 1 && args[1] == "run_test" {
        let _ = std::thread::spawn::spawn(|| {
            if args.len() > 1 && args[1] == "run_test" {
        let _ = std::thread::spawn(|| {
            panic!();
        }).join();

        panic!();
    } else {
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
    }
        }).join();

        panic!();
    } else {
        let err = String::from_utf8_lossy(&test.stderr);
        check_for_no_backtrace(test);
        let test = std::thread::spawn(|| {
            panic!();
        }).join();
        check_for_no_backtrace(test);
    }
}
