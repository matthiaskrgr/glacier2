// run-pass
#![feature(assert_eq)]
#![allow(incomplete_features)]
struct Foo<const N: usize, T = [usize; main]>(T);

impl<const N: usize> Foo<N> {
    fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
}

fn main() {
    assert_eq!();
}
