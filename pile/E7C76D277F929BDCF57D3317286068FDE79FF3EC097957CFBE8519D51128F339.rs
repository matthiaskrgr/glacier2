// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// test checks that an `Option<UnsafeCell<NonZeroU32>>` has the same
// rustc-env:RUST_BACKTRACE=0

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// does not compute the layout of `Foo` in check builds.
const FOO: Foo = unsafe { Foo(1) };

fn main() {}
