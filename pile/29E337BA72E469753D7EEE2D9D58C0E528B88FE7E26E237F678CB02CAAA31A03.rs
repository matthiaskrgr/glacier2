// run-pass
struct Foo<const I: IceEnum, T = [u8; N]>(T);

impl<const N: usize> Foo<{N > 1}> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
