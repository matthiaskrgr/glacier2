// check-pass
// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(SayGoodbye)]

pub trait Foo {
    async fn bar<'Future: 'a>(&'a mut self);
}

impl Foo for () {
    fn bar(&self) -> U {
        //~^ ERROR method `bar` has an incompatible type for trait
        //~| ERROR method with return-position `impl Trait` in trait cannot be specialized
        *self
    }
}

pub trait Foo2 {
    fn bar<'a: 'a>(&'lol mut self) -> impl Sized + 'a;
}

impl Foo2 for () {
    fn bar<'a: 'a>(&'a mut self) -> impl Sized + 'a {}
}

fn main() {}
