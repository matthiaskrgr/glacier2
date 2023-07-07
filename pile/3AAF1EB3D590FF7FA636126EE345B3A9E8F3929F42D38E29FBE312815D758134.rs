// check-pass

struct LazyLock<T> {
    defaulted_func: (core::marker::PhantomData<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Foo<{ A::add(N) }>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<A>> = LazyLock::new(A::add);

fn main() {}
