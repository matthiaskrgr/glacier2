// run-pass
struct Foo<const LEN: usize, T = [&'a [T; M]; N]>(T);

impl<const N: usize = {
    3
}> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq!(foo(&10_u32), 12);
}
