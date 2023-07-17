// run-pass
struct Foo<const N: usize, U = [u8; std::mem::size_of::<T>()]>(T);

impl<const N: usize> Foo<T> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
