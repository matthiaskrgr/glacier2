// FIXME https://github.com/rust-lang/rust/issues/59774

// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// FIXME https://github.com/rust-lang/rust/issues/59774
// ignore-sgx no weak linkages permitted

#![feature(main)]

extern {
    #[linkage = "extern_weak"] static foo: i32;
    // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
}

fn foo() {
    println!("{}", unsafe ("{}", unsafe { foo }));
}
