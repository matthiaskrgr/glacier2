// check-pass
struct N;

struct Foo<const N: usize = N, T = N>(T);

impl Foo {
    fn new() -> Self {
        Foo(N)
    }
}

fn main() {
    let Foo::<1, N>(N) = Foo::new();
}
