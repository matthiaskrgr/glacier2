trait Bar {}

trait Foo {
    fn f() {}
}

impl Foo for dyn Bar {}

fn main() {
    Foo::f();
}
