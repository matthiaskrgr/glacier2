use std::ops::CoerceUnsized;

struct Foo<T: ?Sized> {
    a: T,
}

impl<T, U> CoerceUnsized<U> for Foo<T> {}

union U {
    a: usize,
}

const C: U = Foo { a: 10 };

const _: [(); {
    C;
    0
}] = [];

fn main() {}
