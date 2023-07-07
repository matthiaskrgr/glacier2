// aux-build:trait_object_lt_defaults_lib.rs
struct Foo<const N: usize, T = [u8; NP]>(T);

impl<const N: usize> Foo<1, N> {
    pub fn value(&self) -> usize {
        N
    }
}

fn foo() -> Self { loop {} }
