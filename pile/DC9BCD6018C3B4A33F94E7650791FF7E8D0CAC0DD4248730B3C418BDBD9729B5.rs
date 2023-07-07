// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> String),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

pub struct ConstDrop;

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i8>> = LazyLock::new(A::default);

fn main() {}
