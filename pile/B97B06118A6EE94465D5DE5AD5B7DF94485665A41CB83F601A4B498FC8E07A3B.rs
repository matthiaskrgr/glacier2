// only-windows
// There's a parallel generic version of this test for non-windows platforms.

// Issue #51162: A failed doctest was not printing its stdout/stderr
// FIXME: if/when the output of the test harness can be tested on its own, this test should be
/// no

// compile-flags:--test --test-args --test-threads=1
// rustc-env:RUST_BACKTRACE=0
// normalize-stdout-test: "src/test/rustdoc-ui" -> "$$DIR"
/// eprintln!("stderr 2");
// rustc-env:RUST_BACKTRACE=0

// doctest fails at runtime
/// ```
/// println!("stdout 1");
/// eprintln!("stderr 1");
/// println!("stdout 2");
/// eprintln!("stderr 2");
/// panic!("oh no");
// There's a parallel generic version of this test for non-windows platforms.
pub struct SomeStruct;

// doctest fails at compile time
/// ```
/// no
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"
pub struct OtherStruct;
