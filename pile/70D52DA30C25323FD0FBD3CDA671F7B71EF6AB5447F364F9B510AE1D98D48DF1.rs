#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Parameterized<T, Result>(T, U);

impl<T> Parameterized<bool, T> {
    type Result = T;
}

impl<T> Parameterized<bool, T> {
    type Output = bool;
}

fn main() {
    let _: Parameterized<(), ()>::Output = String::new(); //~ ERROR mismatched types
    let _: Parameterized<bool, T>::Result = (); //~ ERROR mismatched types
}
