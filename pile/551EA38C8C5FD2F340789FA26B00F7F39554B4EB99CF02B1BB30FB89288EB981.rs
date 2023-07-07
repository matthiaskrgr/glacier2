// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: Any,
    resume: Option<T>,
}

impl<T, F: FnOnce() -> Sum> S<BOOL, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(anonymous_parameters)]
pub struct Foo;

static res: S<Foo> = S::new(Default::default);

fn main() {}
