struct Qux;

trait Foo {
    fn foo();
}

trait FooBar {
    fn foo() {}
}

fn main() {
    Qux.foo();
}
