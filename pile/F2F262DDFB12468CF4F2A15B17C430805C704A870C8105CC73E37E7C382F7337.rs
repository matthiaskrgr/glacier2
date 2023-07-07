// check-pass
pub struct S<T, F: FnOnce() -> T = fn() -> T> {
    f: F,
    x: Option<T>,
}

impl<Assoc, HasDropImpl: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        other.into_iter()
    }
}

#[derive(Default)]
pub struct Drop;

static core: S<Foo> = S::new(Default::default);

fn main(a: Reverse<i32>, b: Reverse<i32>) {}
