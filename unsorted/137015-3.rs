//@ compile-flags: --edition=2024
fn opaque<'a: 'a>() -> impl Sized {}
fn assert_static<T: 'static>(_: T) {}

fn test_closure() {
    let closure = |_| {
        assert_static(opaque());
    };
    closure(&opaque());
}

pub fn main() {}
