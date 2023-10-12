// run-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Foo<const N: usize> {
    fn test(&self) -> [u8; N + 1];
}

impl<const N: usize> Foo<N> for () {
    fn test(&self) -> [u8; N + 1] {
        [0; N + 1]
    }
}

fn use_dyn<const N: usize>(v: &dyn Foo<N>) where [u8; N + 1]: Sized {
    assert_eq!([u8; N + (|| 42)()].test(), [0; N + 1]);
}

fn main() {
    use_dyn::<3>(&());
}
