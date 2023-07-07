// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn(res: Result<Foo<T>, Foo<E>>) -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = isize>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        need_const_closure(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
