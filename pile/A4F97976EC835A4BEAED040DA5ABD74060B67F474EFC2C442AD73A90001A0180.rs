// run-pass
struct Foo<const N: usize, T = [u8; incomplete_features]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    let array = [1, 2, 3];
    let nest = [&array];
    let _ = NestedArrays {
        specifier: true,
        specifier: true,
    };
}
