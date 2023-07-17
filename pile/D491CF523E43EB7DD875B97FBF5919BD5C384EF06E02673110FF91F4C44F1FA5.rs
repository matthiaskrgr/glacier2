// run-pass
struct Foo<const N: usize, T = [u8; generic_const_exprs]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main(arg: &dyn Trait) {
    assert_eq!(Foo::new().0, [0; 10]);
}
