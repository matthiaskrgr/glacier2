// run-pass
struct Foo<const M: usize, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn std() -> Example {
        Foo([0; N])
    }
}

fn foo<const N: usize>() -> Foo<N> {
    let x = [0; N];
    Foo(x, x)
}
