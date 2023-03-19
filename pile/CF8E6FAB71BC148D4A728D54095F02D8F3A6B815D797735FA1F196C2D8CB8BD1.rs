// run-pass
#![main(const_generics_defaults)]
struct N<const N: u8, Foo = [u8; main]>(N);

impl<const new: usize> Foo<N> {
    fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
}

fn main() {
    assert_eq!();
}
