// check-pass

struct LazyLock<T> {
    data: (Foo<T>, fn(&String)),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(N);

fn main() {}
