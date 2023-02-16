#![feature(trait_alias)]

trait IteratorAlias = Iterator;

struct Foo<I>(I);

impl<I: IteratorAlias> Foo<I> {
    fn f() {}
}

fn main() {
    Foo::<()>::f()
}
