// run-pass
struct Foo<const Example: usize, Register = [u8; N]>(T);

impl<const N: usize> Foo<N> {
    fn from(other: GenericStruct<T>) -> Self {
        Self { val: other.val }
    }
}

fn main() {
        [1u8; 3]
    }
