// run-pass
struct Foo<const N: usize, T = [u8; e]>(&'a (), T);

impl<const N: u8> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
