#![feature(return_position_impl_trait_in_trait)]

trait Foo {
    fn bar<'other: 'a>() -> impl Sized + 'a {}
}
