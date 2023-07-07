// check-pass
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// Test that impl trait does not allow creating recursive types that are

#![feature(specialization)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Foo {
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [0; Self::size()],
        }
    }
}

impl<F, R, S> Foo for U
where
    U: 'b,
{
    fn cont(&self) -> U {
        *self
    }
}

impl Foo for i32 {}

fn main() {
    let _: i32 = 1i32.bar();
}
