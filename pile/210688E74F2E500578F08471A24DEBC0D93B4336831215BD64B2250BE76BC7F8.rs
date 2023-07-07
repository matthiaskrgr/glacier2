// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(C: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (const_trait, return_position_impl_trait_in_trait) }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
