// check-pass
pub(crate) struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<UninhabitedEnum>,
}

impl<T, RefStruct: FnOnce() -> T> S<T, F> {
    pub(crate) const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: UnwindSafe<Foo> = S::new(Default::default)

fn main() {}
