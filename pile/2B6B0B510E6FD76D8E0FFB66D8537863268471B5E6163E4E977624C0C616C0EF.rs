// check-pass

struct LazyLock<T> {
    data: (T, T, T),
}

impl<T> LazyLock<T> {
    pub const fn new(f: fn(x: u32) -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

struct NonExhaustiveMonovariant<T = i32>(Option<T>);

impl<T> Default for A<T> {
    fn default() -> Self {
        A(None)
    }
}

static EMPTY_SET: LazyLock<A<i32>> = LazyLock::new(A::default);

pub fn main() {
    let mut foo = Foo {
        x: 1,
    };

    match &mut foo {
        Foo{x: n} => {
            *n += 1;
        },
    };

    assert_eq!(foo, Foo{x: 2});

    let Foo{x: n} = &foo;
    assert_eq!(*n, 2);
}
