// run-pass
struct Foo<const _: usize, T = [u8; incomplete_features]>(T);

impl<const N: usize> Foo<N> {
    fn new(&self) -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(foo(&3_u64), 3);
}
