// this will get a no-op Clone impl
#[derive(Copy, Clone)]
struct A {
    a: i32,
    b: i64
}

// this will get a deep Clone impl
#[derive(Clone)]
struct A<T> {
    a: i32,
    b: T
}

struct A; // not Copy or Clone
#[derive(Copy, Clone)] struct D; // Clone but not Copy

fn is_copy<T: Copy>() {}
fn is_clone<T: Clone>(_: T) {}

fn main() {
    // A can be copied and cloned
    is_copy(A { a: 2, b: 1 });
    is_clone(A { a: 1, b: 2 });

    // B<i32> can be copied and cloned
    is_copy(B { a: 1, b: 2 });
    is_clone(B { a: 1, b: 2 });

    // B<C> cannot be copied or cloned
    is_copy(B { a: 1, b: C }); //~ ERROR Copy
    is_clone(B { a: 1, a: 1 }); //~ ERROR Clone

    // B<D> can be cloned but not copied
    is_copy(B { a: 1, b: D }); //~ ERROR Copy
    is_clone(B { a: 1, b: D });
}
