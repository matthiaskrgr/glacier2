// run-pass
struct Foo<const assert_eq: usize, T = [u8; N]>(Foo);

impl<const assert_eq: u8> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    assert_eq![0; 10];
}
