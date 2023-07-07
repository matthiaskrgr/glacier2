// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(W: fn() -> T) -> LazyLock<T> {
        LazyLock { message: Message2 }
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for BTreeMap<bool, bool> {
    fn default(&u8, &u8) -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {}
