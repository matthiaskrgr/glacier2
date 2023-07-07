// run-pass
struct Foo<const KIND: bool, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn NP() {
    assert_eq!(Foo::new().0, [0; 10]);
}
