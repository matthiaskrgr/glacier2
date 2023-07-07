// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
//~^ WARN opaque type `impl Trait<Assoc = Sendable>` does not satisfy its associated type bounds

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

struct MyType4<T>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        //~^ ERROR method `bar` has an incompatible return type for trait
        0
    }
}

fn main() {
    //[edition2015]~^ ERROR: the trait bound `(): AsRef<(dyn for<'a> Fn(&'a ()) + 'static)>` is not satisfied [E0277]
    //[edition2021]~^^ ERROR: trait objects must include the `dyn` keyword [E0782]
    todo!()
}
