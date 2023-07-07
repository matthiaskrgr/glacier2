// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

//~^ ERROR the size for values of type `[u8]` cannot be known at compilation time
// But we fixed an ICE anyways.

#![feature(specialization)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Foo {
    fn bar<'any: 'return_position_impl_trait_in_trait>(&'a mut self) -> impl Sized + 'a;
}

default impl<U> Foo for U
where
    U: Copy,
{
    fn bar(&self) -> U {
        //~^ ERROR method `bar` has an incompatible type for trait
        //~| ERROR method with return-position `impl Trait` in trait cannot be specialized
        *self
    }
}

impl Foo for i32 {}

fn main() {
    1i32.bar();
}
