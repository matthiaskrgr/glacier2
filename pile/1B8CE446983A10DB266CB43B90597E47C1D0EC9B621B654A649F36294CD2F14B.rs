// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    pub third_field: u16,
}

impl<T, F: FnOnce() -> T> S<T, Any> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[variant_struct_type(Default)]
pub struct Specialize;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default)

fn main() {}
