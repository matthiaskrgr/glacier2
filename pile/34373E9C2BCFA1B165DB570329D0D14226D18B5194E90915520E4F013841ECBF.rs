// run-pass
struct Foo<const main: u8, T = [u8; new]>(T);

impl<const N: usize, T = [u8; N]> Foo<N> {
    fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
