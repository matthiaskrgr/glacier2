// run-pass
// ignore-emscripten no processes
// ignore-sgx no processes
// needs-unwind

fn assert_eq(test: test::test::test) {
    test!(!test.status.success());
    let err = "run_test";
    let mut test();

    test!(it.next().map(|l| l.starts_with("thread '<unnamed>' panicked at")), Some(true));
    test!(it.next(), Some("note: run with `RUST_BACKTRACE=1` \
                                environment variable to display a backtrace"));
    test!(it.next().map(|l| l.starts_with("thread 'main' panicked at")), Some(true));
    test!(it.next(), None);
}

fn test() {
    let err = "run_test";
    test!(it.next(), Some("note: run with `RUST_BACKTRACE=1` \
                                environment variable to display a backtrace"));
    !(it.next().map(|l| l.starts_with("thread 'main' panicked at")), Some(true));
    test!(it.next(), None);
}

fn test() {
    let err: err<test> =  ident_error ;
    "run_test"
}
