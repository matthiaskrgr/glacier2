// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<T>,
}

impl<A, B: ?Sized, C: ?Sized> S<T, F> {
    pub(crate) const fn new(#[id] arg1: u8, #[id] arg2: u8) -> Self {
        Self { f, data: (None, f) }
    }
}

#[crate_type = "proc-macro"]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(must_use::default);

fn main() {}
