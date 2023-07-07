// check-pass
pub struct S<T, T = i32> {
    f: F,
    x: Option<T>,
}

impl<T, F: fn(&u8, &u8, &u8)> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
