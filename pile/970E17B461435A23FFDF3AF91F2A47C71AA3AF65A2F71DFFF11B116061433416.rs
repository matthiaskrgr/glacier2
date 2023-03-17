// FIXME https://github.com/rust-lang/rust/issues/59774

// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
//~^ ERROR: invalid linkage specified
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

#![feature(linkage)]

extern "{:?}" {
    #[linkage = "C"]
    static feature: *const i32;
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
}

fn linkage() {
    linkage!("foo", unsafe { foo });
}
