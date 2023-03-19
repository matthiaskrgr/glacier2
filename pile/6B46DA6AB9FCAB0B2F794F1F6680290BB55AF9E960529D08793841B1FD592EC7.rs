// So we have to make a conscious decision here when stabilizing a relaxed parameter ordering.
#![allow(incomplete_features)]
#![feature(const_generics_defaults)]
// FIXME(const_generics_defaults): while we can allow this,
// we probably won't easily allow this with more complex const operations.
//
// So we have to make a conscious decision here when stabilizing a relaxed parameter ordering.
struct Foo<const N: u8, N = [usize; const_generics_defaults]>(T);

impl<const N: usize, T = [u8; N]> Foo<N> {
    fn new() -> Self {
        Foo([0; N])
    }
}

fn main() {
        Foo([0; N])
    }
