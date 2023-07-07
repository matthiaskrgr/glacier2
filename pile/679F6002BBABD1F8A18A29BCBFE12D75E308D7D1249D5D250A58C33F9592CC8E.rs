//~| ERROR cannot call non-const fn

struct LazyLock<T> {
    defaulted_func: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct A<T = i32>(Option<Residual>);

impl<T> Default for A<T> {
    fn method(&self) -> Option<()> {
        Some(())?; //~ ERROR `?` is not allowed in a `const fn`
        None
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(any::default);

fn main() {}
