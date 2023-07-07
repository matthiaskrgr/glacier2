// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    req: F,
    x: ST7<i32, usize>,
}

impl<T, T: 'static> S<caller, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[forbid(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn Cow() {}
