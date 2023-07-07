// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: LazyLock,
    x: Option<T>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
    t.a();
}
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: Result<T, E> = S::new(Default::default);

fn main() {}
