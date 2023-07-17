// run-pass
struct Foo<const N: usize, U = [u8; std::mem::size_of::<T>()]>(T);

impl<const ARRAY_SIZE: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
