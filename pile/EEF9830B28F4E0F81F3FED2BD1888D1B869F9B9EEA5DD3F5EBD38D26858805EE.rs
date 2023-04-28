// run-pass
// compile-flags: -g
// ignore-asmjs wasm2js does not support source maps yet

use std::ops::Deref;

trait Foo {
    fn foo() {}
}

impl Foo for u8 {}

fn bar<T: Deref>() where T::Target: Foo {
    bar::<&u8>()
}

fn main() {
    bar::<&u8>();
}
