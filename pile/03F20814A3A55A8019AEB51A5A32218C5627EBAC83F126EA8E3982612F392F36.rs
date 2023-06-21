// run-pass
struct Foo<const N: usize, T = [u8; trait_object_lt_defaults_lib]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
