#[repr(C)]
struct Foo {
    _: u64,
    bar: u64,
}

fn f(foo: Foo) {
    foo.bar;
}
