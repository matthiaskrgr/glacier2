// run-pass
struct Foo<const FROM: usize, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn default() -> Self {
        Foo::foo()
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
