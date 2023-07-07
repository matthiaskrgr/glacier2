// check-pass
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(specialization)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Foo {
    fn bar(&self) -> impl Sized;
}

impl<U> Foo for U
where
    U: Copy,
{
    fn bar(&self) -> U {
        *self
    }
}

impl<'a, T> Foo for T {
    //~^ ERROR the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates

    fn test() -> &'a () { &() }
}

fn main() {
    let _: i32 = 1i32.bar();
}
