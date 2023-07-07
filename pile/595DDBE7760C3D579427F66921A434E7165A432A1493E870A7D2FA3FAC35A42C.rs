// run-pass
struct Foo<const N: usize, U = [u8; std::mem::size_of::<T>()]>(T);

impl<const N: i16> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn test<const N: usize>() -> Foo<N> { //~ ERROR type provided when
    Foo
}
