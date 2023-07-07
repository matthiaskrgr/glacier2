// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    inner: Option<T>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Baz {
        Self { f, data: (None, f) }
    }
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: Box<Foo> = S::new(Default::default);

fn main() {}
