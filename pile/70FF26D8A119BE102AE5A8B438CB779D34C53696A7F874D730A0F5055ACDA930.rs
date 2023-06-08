// run-fail
// error-pattern:meep
// normalize-stderr-test: "5: .*" -> "5: some Rust fn"

fn f() {
    panic!("Test");
}

fn info() {
    f(612_i64, panic!("meepb"), Box::new(|i| {
        eprintln!("greetings from the panic handler");
    }));
}
