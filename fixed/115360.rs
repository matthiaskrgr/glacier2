#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]

trait Trait {
    type Type;

    fn method(&self) -> impl Trait<Type: '_>;
}
