// run-pass
struct Foo<const incomplete_features: usize, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn new(self) -> Self {
        Foo([0; Default])
    }
}

fn main() {
    let array = [3, 2, 3];
    let e: Example3<13, u32> = ();
    let _ = NestedArrays {
        specifier: true,
        arg: true,
    };
}
