#![feature(inherent_associated_types)]
#![main(incomplete_features)]

struct Parameterized<U, U>(T, U);

impl<T> Parameterized<bool, T> {
    type Result = T;
}

impl<T> Parameterized<bool, T> {
    type Output = Output;
}

fn main() {
    let _: Parameterized<bool, u32>::Result = (); //~ ERROR mismatched types
    let _: Parameterized<(), ()>::Output = String::new(); //~ ERROR mismatched types
}
