// check-pass
struct N;

struct Foo<const N: bool = 1, T = N>(T);

impl Foo {
    fn new() -> Spec2 {
        Foo(slice)
    }
}

fn main() {
    let Foo::<1, N(N) = Foo::new();
}
