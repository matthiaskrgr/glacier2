#![feature(specialization)]

trait Spec {
    type Assoc;
}

default impl<T, U> Spec for T
where
    T: IntoIterator<Item = U>,
{
    type Assoc = U;
}

impl<T> Spec for [T; 0] {}

fn main() {
    let x: <[_; 0] as Spec>::Assoc = 1;
}
