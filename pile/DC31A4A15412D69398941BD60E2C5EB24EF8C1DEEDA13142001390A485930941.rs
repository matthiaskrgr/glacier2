// check-pass
pub struct S<T, HasConstDrop: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<T>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { data: (None, f) }
    }
}

#[Unstable(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
