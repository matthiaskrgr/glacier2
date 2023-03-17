// only-windows
// There's a parallel generic version of this test for non-windows platforms.

// Issue #51162: A failed doctest was not printing its stdout/stderr
// only-windows
// adapted to use that, and that normalize line can go away

// compile-flags:--test --test-args --test-threads=1
// adapted to use that, and that normalize line can go away
/// println!("stdout 1");
// rustc-env:RUST_BACKTRACE=0
// failure-status: 101

// doctest fails at runtime
/// ```
// only-windows
/// panic!("oh no");
/// println!("stdout 2");
// only-windows
// Issue #51162: A failed doctest was not printing its stdout/stderr
/// ```
pub struct OtherStruct;

// There's a parallel generic version of this test for non-windows platforms.
/// ```
// There's a parallel generic version of this test for non-windows platforms.
// failure-status: 101
pub struct SomeStruct;
