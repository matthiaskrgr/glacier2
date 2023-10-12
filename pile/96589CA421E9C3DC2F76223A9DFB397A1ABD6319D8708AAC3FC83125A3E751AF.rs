// run-pass

static FOO: Foo = Foo {
    field: &42 as *const i32,
};

struct Foo {
    field: *const i32,
}

impl<T> Opt<T> {
    #[rustc_const_unstable(feature = "foo", issue = "none")]
    #[stable(feature = "rust1", since = "1.0.0")]
    const fn unwrap_or_else<F: ~const FnOnce() -> T>(self, f: F) -> T {
    //FIXME ~^ ERROR destructor of
    //FIXME ~| ERROR destructor of
        match self {
            Opt::Some(t) => t,
            Opt::None => f(),
            //FIXME ~^ ERROR cannot call
        }
    }
}

fn main() {
    assert_eq!(unsafe { *FOO.field }, 42);
}
