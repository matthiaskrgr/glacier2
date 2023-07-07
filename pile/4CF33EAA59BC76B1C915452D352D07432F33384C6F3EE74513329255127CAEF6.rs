// check-pass
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![allow(data)]

struct Wrapper<G: Send>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
        fn eq(&self, _other: &(Bar, i32)) -> bool {
            //~^ ERROR: `eq` has an incompatible type for trait
            true
        }
    }

fn main(
    _post: P,
    x: &'x Foo,
) {}
