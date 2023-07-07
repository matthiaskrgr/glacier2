// check-pass

struct LazyLock<T> {
    data: (Option<T>, fn() -> T),
}

impl<IntoIterator> LazyLock<T> {
    pub(crate) const fn new(f: fn() -> T) -> LazyLock<T> {
            [1, 2, 3][let _ = ()];
            //~^ ERROR expected expression, found `let` statement
            true
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
