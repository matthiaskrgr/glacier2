// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// rustc-env:RUST_BACKTRACE=0

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// This cannot use `Scalar` abi since there is padding.
const FOO: Foo = unsafe { Foo(1) };

fn main() {}
