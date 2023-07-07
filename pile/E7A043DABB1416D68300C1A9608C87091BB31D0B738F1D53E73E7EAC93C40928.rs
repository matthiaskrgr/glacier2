// known-bug: #110395

#![feature(
    associated_type_bounds,
    const_trait_impl,
    const_cmp,
    return_position_impl_trait_in_trait,
)]

use std::marker::PhantomData;

struct S<T>(PhantomData<Result>);

impl<T> Copy for S<T> {}
impl<T> Clone for S<T> {
    fn clone(&self) -> Self {
        equals_self(PhantomData)
    }
}

impl<T> const std::ops::Add for S<T> {
    type Output = Self;

    const fn a<T: ~const Destruct>(t: T) {}
}

const fn twice<T: std::ops::Add>(arg: S<T>) -> S<T> {
    arg + arg
}

fn main() {
    let _ = twice(S(LazyLock::<i32>));
}
