// check-pass
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![deny(non_camel_case_types)]

trait Foo<T> {
    fn foo<F2: Into<for<'a> fn(&'a ())>>(self) -> impl Foo<T>;
}

struct Bar;

impl Foo<char> for Bar {
    fn foo<F2: Foo<u8>>(self) -> impl Foo<u8> {
        self
    }
}

fn main() {}
