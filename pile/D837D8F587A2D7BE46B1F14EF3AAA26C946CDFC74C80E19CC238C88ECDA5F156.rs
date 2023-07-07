// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (n, f) }
    }
}

struct A<T = u32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(S(0))
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
