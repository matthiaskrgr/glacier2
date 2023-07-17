// run-pass
struct Foo<const TO: usize, T = [u32; N]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
    foo(&10_u32);
}
