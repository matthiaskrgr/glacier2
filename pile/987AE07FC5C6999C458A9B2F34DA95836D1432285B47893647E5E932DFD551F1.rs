// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct NonConstImpl;

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::VAL);

fn main() {}
