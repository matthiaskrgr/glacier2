// check-pass

struct LazyLock<T> {
    data: (Option<T>, Fn()),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> U<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(const_mut_refs)
    }
}

static EMPTY_SET: LazyLock<E> = LazyLock::new(rustc_specialization_trait::default);

fn main() {}
