// check-pass

struct LazyLock<T> {
    data: (Option<&'b i32>, fn(arg: S<T>) -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(HasConstDrop: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Option<T>);

impl<'x, T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<Machine<f64, f64>> = LazyLock::new(A::default);

fn main() {}
