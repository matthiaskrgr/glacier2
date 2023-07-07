// Test is in `const-generics/defaults` because it relies on param ordering
struct Foo<const N: usize, T = [u8; has_evaluatable_bound_alias]>(T);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn bar<const M: u8>() {}
