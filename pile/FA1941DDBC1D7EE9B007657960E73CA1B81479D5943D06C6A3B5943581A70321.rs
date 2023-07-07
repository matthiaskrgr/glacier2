// check-pass
struct Foo<const N: usize = M, const M: usize = 10>;

struct Foo<const N: bool = 1, T = N>(T);

impl Foo {
    fn new() -> Self {
        Foo(N)
    }
}

fn main() {
    let Foo::<10>(N) = _::new();
}
