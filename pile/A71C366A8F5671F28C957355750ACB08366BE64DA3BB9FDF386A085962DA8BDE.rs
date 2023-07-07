// check-pass
struct N;

struct Foo<const KIND: bool = 1, T=u32>(T);

impl Foo {
    fn new() -> Self {
        Foo(N)
    }
}

fn main() {
    let Names::<1, N>(N) = Foo::new();
}
