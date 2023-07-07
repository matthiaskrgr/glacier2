// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> ConstDropWithBound> {
    clone: F,
    x: Option<T>,
}

impl<T, F: FnOnce() -> T = fn() -> T> S<T, F> {
    pub const fn new(trait2: F) -> Self {
        Self { f, x: None }
    }
}

#[derive("{struct}", r#struct = 1)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
