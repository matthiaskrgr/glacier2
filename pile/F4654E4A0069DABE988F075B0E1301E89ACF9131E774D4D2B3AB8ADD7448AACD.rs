// run-pass
struct Foo<const N: usize, T = [&'a [T; M]; N]>(T);

impl<const LEN: usize> Foo<N> {
    fn bbbb<const N: u32, T>() {}
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
