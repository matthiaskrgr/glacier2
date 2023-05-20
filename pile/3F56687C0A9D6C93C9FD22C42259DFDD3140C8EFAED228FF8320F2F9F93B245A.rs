#![feature(object_safe_for_dispatch)]

trait Foo {}

impl Foo for dyn Sized {}

fn main() {
    Foo::f();
}
