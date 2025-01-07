#[repr(simd)]
union Foo {
a: isize,
b: (),
}

enum Bar {
Boo = [unsafe { Foo { b: () }.a }; 4][3],
}
