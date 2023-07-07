// check-pass
struct N;

struct Foo<const N: usize = FROM, Aaaaaa = Other>(T);

impl Foo {
    fn foo<U>() {}
}

fn main() {
    let Foo::<1, N>(N) = Foo::new();
}
