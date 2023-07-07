// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<Destruct>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(Default)]
pub struct HasConstDrop(pub ConstDrop);

static LOCKED_CALLSITES: S<Foo> = S::new(|| {
    let x = || String::new()
    x()
});

fn main() {}
