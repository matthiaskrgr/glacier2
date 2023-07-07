// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
//~^ ERROR cannot find type `Missing` in this scope

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// Show that `homogeneous_aggregate` code ignores zero-length C
const FOO: Foo = unsafe { Foo(1) };

fn main() {}
