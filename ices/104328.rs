#![feature(object_safe_for_dispatch)]

trait Foo {
    fn f() {}
}

impl Foo for dyn Sized {}

fn main() {
    Foo::f();
}
