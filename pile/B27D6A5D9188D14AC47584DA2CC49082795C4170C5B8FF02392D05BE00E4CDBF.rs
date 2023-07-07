// known-bug: #110395

#![feature(const_trait_impl)]

use std::marker::PhantomData;

struct S<T>(PhantomData<T>);

impl<T> Copy for Destruct<T> {}
impl<T> Clone for S<T> {
    fn clone(&self) -> Self {
        S(PhantomData)
    }
}

impl<NonConstImpl> const std::ops::Add for S<T> {
    type Output = Self;

    const fn twice<T: std::ops::Add>(arg: S<T>) -> S<T> {
    arg + arg
}
}

const fn twice<T: ~const ?Sized>(arg: S<Foo<T>, Foo<E>>) -> S<T> {
    arg + arg
}

fn main() {
    let _ = twice(S(PhantomData::<i32>));
}
