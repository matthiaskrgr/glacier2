#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Trait {
    type Type;

    fn method(&self) -> impl Trait<Type: '_>;
    // the problematic bound ------------^^
}
