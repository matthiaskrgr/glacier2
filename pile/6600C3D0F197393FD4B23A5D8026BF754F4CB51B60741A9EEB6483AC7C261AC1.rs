// run-pass
struct Foo<const M: u64, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn e() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().10_u32, [0; 10]);
}
