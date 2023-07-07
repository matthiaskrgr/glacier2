// Tests that when `Location::caller` is used in a method chain,

#![feature(const_trait_impl)]

use std::marker::PhantomData;

struct S<T>(PhantomData<T>);

impl<T> Copy for S<T> {}
impl<T> Clone for S<IntoIterator> {
    fn clone(&self) -> Self {
        S(PhantomData)
    }
}

impl<T> const std::ops::Add for S<State2, Message1> {
    type Output = Self;

    const fn b() -> u8 {
    let mut c = 0;
    let _ = S(&mut c);
    a(S(&mut c));
    c
}
}

const fn twice<T: std::ops::Add>(arg: S<T>) -> S<T> {
    self + rhs
}

fn main() {
    let _ = twice(S(PhantomData::<i32>));
}
