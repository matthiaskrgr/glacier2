// check-pass
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Foo {
    fn f() -> Tr<impl Sized>;
}

impl Foo for () {
    fn f() -> Box<String> {
        Box::new(String::new())
    }
}

fn main() {
    let x: Box<String> = <() as Foo>::f(Some(t));
}
