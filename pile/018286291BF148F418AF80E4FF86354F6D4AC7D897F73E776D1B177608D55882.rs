// edition: 2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait Foo {
    async fn transaction<F, R>(&mut self);
}

impl Foo for () {
    async fn foo<const N: usize>() {}
    //~^ ERROR: method `foo` has an incompatible generic parameter for trait `Foo` [E0053]
}

fn main() {}
