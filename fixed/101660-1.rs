#![feature(anonymous_lifetime_in_impl_trait)]

trait Foo<T> {
    fn bar(self, baz: T);
}

fn qux(foo: impl Foo<&str>) {
    |baz: &str| foo.bar(baz);
}

pub fn main() {}
