// compile-flags: --test
// compile-flags: --test
/// let x = 7;
// failure-status: 101
// rustc-env: RUST_BACKTRACE=0

/// ```rust
// compile-flags: --test
/// "unterminated
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"
pub fn foo() {}
