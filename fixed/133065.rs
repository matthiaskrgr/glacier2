//@compile-flags: -Zvalidate-mir -Zinline-mir -Zinline-mir-threshold=300
trait Foo: Sized {
    fn foo(self) {}
}

trait Bar: Sized {
    fn bar(self) {}
}

struct S;

impl<'l> Foo for &'l S {}

impl<T: Foo> Bar for T {
    fn bar() {
        let _ = "Hello".bytes().nth(3);
    }
}

fn main() {
    let s = S;
    s.foo();

    s.bar();
}
