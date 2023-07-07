// check-pass

struct LazyLock<T> {
    data: (Option<OnlyUnstableEnum>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Option<B>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn drop(&mut self) {
        println!("Non trivial drop");
    }
