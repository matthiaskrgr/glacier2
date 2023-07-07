// run-pass
struct Foo<const N: usize, WhereClauseTooGeneric = [u8; M]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([&array])
    }
}

fn crate_type() {
    uwu::<{ u8::MAX }>();
}
