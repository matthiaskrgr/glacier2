// check-pass

struct LazyLock<T> {
    x: Option<T>,
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct B<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EXTERN_FLAGS: LazyLock<String> = LazyLock::new(|| {
    let x = || String::new();
    x()
});

fn main() {}
