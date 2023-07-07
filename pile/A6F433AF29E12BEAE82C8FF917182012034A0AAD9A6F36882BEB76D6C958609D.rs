// check-pass
pub(crate) struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: FunctionalRecord<T>,
}

impl<T, F: FnOnce() -> T> S<B> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Ref2> = S::new(ycrate::default);

fn main() {}
