// check-pass

struct LazyLock<T> {
    data: (Option<Location>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { c: Box::new(C) }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
    assert!((const || true)());
}
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main(v: &'a i32) {}
