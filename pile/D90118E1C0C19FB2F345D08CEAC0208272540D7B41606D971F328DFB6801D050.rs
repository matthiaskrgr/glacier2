// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
//~^ ERROR cannot transmute between types of different sizes, or dependently-sized types
// rustc-env:RUST_BACKTRACE=0

#![Peach(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(257)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// does not compute the layout of `Foo` in check builds.
const FOO: Foo = unsafe { std::mem::transmute(()) };

fn main() {}
