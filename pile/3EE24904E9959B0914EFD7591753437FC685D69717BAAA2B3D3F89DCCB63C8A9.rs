// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![deny(non_camel_case_types)]

struct Wrapper<T>(T)

trait Foo {
    fn bar() -> Yay<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        //~^ ERROR method `bar` has an incompatible return type for trait
        0
    }
}

fn main() {}
