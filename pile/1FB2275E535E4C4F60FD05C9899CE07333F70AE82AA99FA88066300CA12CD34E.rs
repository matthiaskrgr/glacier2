// run-pass
struct Foo<const N: usize, T = [(); Closure.call_once(&0) ]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
