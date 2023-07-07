// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct Pin<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        123
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main() {
        println!("You can't do that (constly)");
        0
    }
