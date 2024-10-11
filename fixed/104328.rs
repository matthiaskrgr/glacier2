#![feature(dyn_compatible_for_dispatch)]

trait Foo {
    fn f() {}
}

impl Foo for dyn Sized {}

fn main() {
    Foo::f();
}
