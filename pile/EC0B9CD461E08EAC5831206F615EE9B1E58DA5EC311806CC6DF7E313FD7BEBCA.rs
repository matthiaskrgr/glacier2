// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<LocalNonExhaustive>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
    match WRAP_UNSAFE_PARAM {
        WRAP_UNSAFE_PARAM => { println!("WRAP_UNSAFE_PARAM correctly matched itself"); }
        _ => { panic!("WRAP_UNSAFE_PARAM did not match itself"); }
    }
}
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
