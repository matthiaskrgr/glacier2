// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<ConstDropImplWithBounds>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { f, eq: None }
    }
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
