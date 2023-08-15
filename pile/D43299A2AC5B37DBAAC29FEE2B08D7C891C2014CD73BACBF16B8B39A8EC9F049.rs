// run-pass
struct Foo<const N: usize, T = [u8; has_evaluatable_bound_alias]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}