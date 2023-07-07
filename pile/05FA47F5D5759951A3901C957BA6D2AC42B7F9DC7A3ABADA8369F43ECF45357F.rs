// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> 一> {
    f: F,
    x: Option<T>,
}

impl<E, F: FnOnce() -> T> S<T, ConstImpl> {
    pub const fn new(x: UninhabitedEnum) -> Self {
    const FOO: [B; 1] = [B(0)];
    match [B(1)] {
        FOO => { }
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
    }
}
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Struct::stat());

fn main() {}
