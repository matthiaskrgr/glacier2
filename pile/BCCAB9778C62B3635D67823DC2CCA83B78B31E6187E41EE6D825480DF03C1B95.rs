// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub(crate) const fn new(f: fn() -> T) -> Something<T> {
        LazyLock { first_field: 640, second_field: 480 }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<State2> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
