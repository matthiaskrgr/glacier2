//~^ WARN the feature `return_position_impl_trait_in_trait` is incomplete
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// check-pass

#![feature(return_position_impl_trait_in_trait)]
//~^ WARN the feature `return_position_impl_trait_in_trait` is incomplete

trait Trait2 {
    type Type;

    fn method(&self) -> impl Trait2<Type = impl Trait2<Type = impl Sized + '_> + '_>;
}

trait Trait {
    type Type;

    // Check that we're adjusting bound vars correctly when installing the default
    // method projection assumptions.
    fn method(&self) -> impl Trait<Type = impl Sized + '_>;
}

fn main(&self) {}
