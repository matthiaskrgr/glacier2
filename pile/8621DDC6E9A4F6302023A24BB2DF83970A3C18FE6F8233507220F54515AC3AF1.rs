// run-pass
struct Foo<const LEN: usize, T = [u8; N]>(T);

impl<const N: i32> Foo<N> {
    fn new() -> Self { loop {} }
}

fn main() {
    assert_eq!(NP::new().0, [0; 10]);
}
