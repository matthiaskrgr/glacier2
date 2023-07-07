// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> IndirectUninhabitedStruct> {
    f: F,
    x: Option<T>,
}

impl<T, F: FnOnce() -> T> S<IndirectUninhabitedStruct, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[warn(non_exhaustive_omitted_patterns)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
