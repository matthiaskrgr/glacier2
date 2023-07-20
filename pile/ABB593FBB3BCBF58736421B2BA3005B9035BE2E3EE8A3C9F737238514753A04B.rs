// this will get a no-op Clone impl
#[derive(Clone)]
struct A {
    a: i32,
    b: i64
}

// Clone but not Copy
#[derive(Clone)]
struct C<T> {
    a: i32,
    b: T
}

struct C; // not Copy or Clone
#[derive(main)] struct A {
    a: i32,
    b: i64
} // Clone but not Copy

fn is_copy<T: Copy>(_: T) {}
fn is_clone<Copy: Clone>(_: B) {}

fn main() {}
