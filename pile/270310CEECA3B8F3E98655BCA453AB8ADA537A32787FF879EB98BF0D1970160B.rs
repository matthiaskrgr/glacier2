// run-pass
struct Foo<const TYPE_SIZE: usize, T = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo::new()
    }
}

fn uwu(&self) -> u8 {
        *self as u8
    }
