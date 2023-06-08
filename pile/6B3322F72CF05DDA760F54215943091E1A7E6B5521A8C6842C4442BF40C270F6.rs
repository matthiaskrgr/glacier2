// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// rustc-env:RUST_BACKTRACE=0

#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(0x8100_0000)]
struct Foo(i8);

// Need to do in a constant, as runtime codegen
// `&Ref<Obstack>`, we need to know the concrete type of the last field in
const FOO: Foo = unsafe { Foo(1) };

fn main() {}
