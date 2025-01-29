#![feature(min_generic_const_args)]
static A: u32 = 0;
struct Foo<const N: u32>;
const B: Foo<{ A }> = Foo;
fn main() {}
