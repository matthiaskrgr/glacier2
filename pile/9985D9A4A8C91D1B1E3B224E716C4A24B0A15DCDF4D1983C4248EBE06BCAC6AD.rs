// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn() -> Something) -> LazyLock<T> {
        self + rhs
    }
}

struct A<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(CFN6)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

fn main(#[a1] self, #[a2] arg1: u8) {}
