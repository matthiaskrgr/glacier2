#![feature(return_position_impl_trait_in_trait)]
#[allow(incomplete_features)]

struct S;

trait Foo {
    fn bar<T>() -> impl Sized;
}

impl Foo for S {
    fn bar() -> impl Sized {}
}

fn main() {
    S::bar();
}
