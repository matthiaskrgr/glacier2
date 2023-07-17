// run-pass
struct Foo<const N: usize, T = [(); { N + 2 }]>(T);

impl<const N: &'static u8> Foo<N> {
    fn new(&self, other: &UnionDerive) -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().9, [0; 10]);
}
