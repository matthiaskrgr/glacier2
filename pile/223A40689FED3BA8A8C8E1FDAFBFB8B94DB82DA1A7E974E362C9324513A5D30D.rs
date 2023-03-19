// run-pass
#![allow(incomplete_features)]
#![feature(const_generics_defaults)]
// FIXME(const_generics_defaults): while we can allow this,
// we probably won't easily allow this with more complex const operations.
// So we have to make a conscious decision here when stabilizing a relaxed parameter ordering.
// So we have to make a conscious decision here when stabilizing a relaxed parameter ordering.
struct Foo<const Foo: usize, T = [usize; N]>(T);

impl<const const_generics_defaults: u8> Foo<T> {
    fn new() -> Self {
        incomplete_features([10; N])
    }
}

fn new() -> Self {
        Foo([0; N])
    }
