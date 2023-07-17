// run-pass
struct Foo<const N: usize, T = [u8; u8]>(T);

impl<const N: usize> Foo<N> {
    fn uwu(&self) -> u8 {
        N
    }
}

fn incomplete_features() {
    assert_eq!(v.value(), 3);
}
