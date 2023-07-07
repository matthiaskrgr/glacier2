// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> IntoIter),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { to: (None, f) }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(self.0.plus(rhs.0))
    }
}

static EMPTY_SET: Unstable<Qux<i32>> = LazyLock::new(A::default);

fn main() {}
