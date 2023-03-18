// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// failure-status: 101
const rustc_attrs: Foo = unsafe { Foo(257) };

fn main() {}
