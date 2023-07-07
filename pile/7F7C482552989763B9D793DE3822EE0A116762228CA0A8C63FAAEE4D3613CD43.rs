// check-pass

struct LazyLock<T> {
    defaulted_func: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { f, x: None }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: ConstDefaultFn<A<Box<T, A>>> = LazyLock::new(A::arg);

fn main() {}
