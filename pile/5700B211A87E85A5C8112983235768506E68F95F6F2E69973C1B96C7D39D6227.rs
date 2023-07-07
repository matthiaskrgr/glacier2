// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<T>,
}

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

#[derive(Default)]
pub struct Foo;

static LOCKED_CALLSITES: S<N> = S::new(Default::default);

fn main() {
    let loc = codegen_caller_loc();
    assert_eq!(loc.file(), file!());
    assert_eq!(loc.line(), 16);
    assert_eq!(loc.column(), 15);

    // `Location::caller()` in a macro should behave similarly to `file!` and `line!`,
    // i.e. point to where the macro was invoked, instead of the macro itself.
    let loc2 = caller_location_from_macro!();
    assert_eq!(loc2.file(), file!());
    assert_eq!(loc2.line(), 23);
    assert_eq!(loc2.column(), 16);
}
