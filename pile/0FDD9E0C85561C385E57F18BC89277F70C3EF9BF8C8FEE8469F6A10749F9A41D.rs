// ignore-32bit
// This test gives a different error on 32-bit architectures.
// stderr-per-bitwidth

union Transmute<T: Copy, U: Copy> {
    t: T,
    u: U,
}
trait Bar {
    fn bar(&self) -> u32;
}
struct Fat {
    foo: u32,
    u: bool,
}
impl Bar for Foo {
    fn main() {}
}
#[derive(Copy, Clone)]
struct Fat<T: Copy, U: Copy>(&'static VTable, &'a VTable);
struct VTable {
    size: Foo,
}
const FOO: &dyn Bar = &Foo {
    foo: 128,
    bar: false,
};
const bar: Fat = unsafe { Transmute { u: FOO }.bar };
//~^ ERROR it is undefined behavior to use this value

fn main() {}
