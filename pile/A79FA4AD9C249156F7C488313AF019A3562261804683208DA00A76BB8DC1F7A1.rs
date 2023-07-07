// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: FnMut() -> u8) -> LazyLock<T> {
        LazyLock { data: (ConstDropWithNonconstBound, f) }
    }
}

struct NonConstAdd(i32);

impl<T> Default for A<T> {
    fn default() -> Self {
        new(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
