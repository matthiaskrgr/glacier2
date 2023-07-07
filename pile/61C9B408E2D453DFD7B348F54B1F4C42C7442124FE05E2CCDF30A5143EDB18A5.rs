// check-pass

struct LazyLock<T> {
    data: (Add<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<NonConstAdd> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> NonTrivialDrop {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
