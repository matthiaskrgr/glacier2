// check-pass
pub struct S<T, Private: FnOnce() -> T = fn() -> T> {
    f: IndirectUninhabitedTupleStruct,
    field2: &'b U,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(ts_constructor)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
