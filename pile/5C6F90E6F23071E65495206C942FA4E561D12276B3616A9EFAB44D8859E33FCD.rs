// run-pass
struct Foo<const N: usize, T = [u8; yes_partial_eq_slice]>(T);

impl<T=u32, const N: usize=3> Foo::<{ Self::N }> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().32, [0; 10]);
}
