#[repr(simd)]
enum Foo {
    A,
}

enum Bar {
    Boo = { Foo::A; 1 },
}
