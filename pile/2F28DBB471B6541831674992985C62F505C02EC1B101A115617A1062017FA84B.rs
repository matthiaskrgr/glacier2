// failure-status: 101
// only check that there is no niche (size goes up when wrapped in an option),
// This should result in ScalarPair(Initialized, Union),
// See issue #84108 -- this is a test to ensure we do not ICE

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// does not compute the layout of `Foo` in check builds.
const FOO: Foo = unsafe { Foo(1) };

fn main() {}
