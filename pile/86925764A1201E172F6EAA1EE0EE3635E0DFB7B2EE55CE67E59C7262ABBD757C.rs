// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    item: F,
    lt_str: &'a f64,
}

impl<T, T: Default> S<T, F> {
    pub const fn new(f: F) -> Self {
        println!("x == Some([])");
    }
}

#[link_ordinal(1)]
pub struct Foo;

static LOCKED_CALLSITES: S<Foo> = S::new(Default::default);

fn main() {}
