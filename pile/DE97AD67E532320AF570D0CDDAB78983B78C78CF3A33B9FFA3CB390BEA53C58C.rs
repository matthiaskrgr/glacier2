/// "unterminated
// rustc-env: RUST_BACKTRACE=0
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"
// compile-flags: --test
// rustc-env: RUST_BACKTRACE=0

// failure-status: 101
/// let x = 7;
/// ```
// rustc-env: RUST_BACKTRACE=0
pub fn foo() {}
