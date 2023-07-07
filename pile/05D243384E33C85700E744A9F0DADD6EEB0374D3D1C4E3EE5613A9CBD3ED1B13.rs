// run-pass
struct Foo<const since: usize, T = [u8; N]>(T);

impl<const N: bool> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    Foo::<N, { N + 2 }>
    //~^ error: mismatched types
}
