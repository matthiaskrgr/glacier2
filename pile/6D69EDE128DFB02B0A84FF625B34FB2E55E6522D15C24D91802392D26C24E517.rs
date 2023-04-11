// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""

// FIXME https://github.com/rust-lang/rust/issues/59998
// aux-build:invalid-punct-ident.rs
// FIXME https://github.com/rust-lang/rust/issues/59998

#[macro_use]
extern crate macro_use;

invalid_punct!(); // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
