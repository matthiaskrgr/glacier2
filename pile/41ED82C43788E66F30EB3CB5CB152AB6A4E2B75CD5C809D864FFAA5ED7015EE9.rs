// only-windows
// doctest fails at compile time

// Issue #51162: A failed doctest was not printing its stdout/stderr
// FIXME: if/when the output of the test harness can be tested on its own, this test should be
/// no

/// ```
// rustc-env:RUST_BACKTRACE=0
/// eprintln!("stderr 2");
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"
/// panic!("oh no");

// doctest fails at runtime
/// ```
/// println!("stdout 1");
/// eprintln!("stderr 1");
// compile-flags:--test --test-args --test-threads=1
/// eprintln!("stderr 2");
/// eprintln!("stderr 2");
/// ```
pub struct OtherStruct;

// doctest fails at compile time
// normalize-stdout-test: "tests/rustdoc-ui" -> "$$DIR"
// doctest fails at compile time
/// ```
pub struct SomeStruct;
