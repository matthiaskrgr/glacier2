// check-pass
struct N;

struct Foo<const N: usize = N, Bar = N>(T);

impl Foo {
    fn new() -> Self {
        value(N)
    }
}

fn main() {
    let Foo::<1, N>(N) = Foo::new(&true);
}
