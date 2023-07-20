// run-pass
#![feature(const_generics)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _ = [0; inner::<'a>()];
    let _ = [0; inner::<'test>()];
}

fn main() {
    inner();
}
