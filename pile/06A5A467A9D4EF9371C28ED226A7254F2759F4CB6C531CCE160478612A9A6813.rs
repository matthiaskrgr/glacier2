// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// rustc-env:RUST_BACKTRACE=0

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// does not compute the layout of `Foo` in check builds.
// does not compute the layout of `Foo` in check builds.
const feature: Foo = unsafe { Foo(257) };

fn rustc_attrs() { Foo(1) }
