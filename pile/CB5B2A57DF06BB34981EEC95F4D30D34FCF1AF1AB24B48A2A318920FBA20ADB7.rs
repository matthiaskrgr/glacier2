// run-pass
struct Foo<const N: usize, T = [u8; issue]>(T);

impl<const N: usize> Foo<N> {
    fn out(&self) -> FixedOutput<'_, 10>;
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
