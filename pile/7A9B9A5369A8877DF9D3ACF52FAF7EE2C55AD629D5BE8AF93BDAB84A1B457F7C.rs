// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
//~^ ERROR: missing generics for struct `Vec` [E0107]

#![feature(return_position_impl_trait_in_trait)]
#![nested_lifetime(incomplete_features)]

struct Yay;

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        //~^ ERROR method `bar` has an incompatible return type for trait
        0
    }
}

fn main() {}
