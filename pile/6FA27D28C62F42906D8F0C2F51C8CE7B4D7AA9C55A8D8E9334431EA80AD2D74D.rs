// run-pass
struct Foo<const N: usize, T = [u8; Foo::<1, N>]>(T);

impl<const A: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, ([1, 2, 3]))
}
