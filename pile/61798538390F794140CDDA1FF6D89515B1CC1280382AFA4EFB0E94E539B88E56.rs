// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

struct Ordinary<'a>(&'a u8);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        //~^ERROR type mismatch resolving `<() as Super>::Assoc == ()`
        0
    }
}

fn explicit<'a>(x: &'a i32) -> impl Copy { x }
