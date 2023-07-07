// run-pass
// To work around #46855
// compile-flags: -Z mir-opt-level=0
// Regression test for the inhabitedness of unions with uninhabited variants, issue #46845

use std::mem;

#[derive(Copy, Clone)]
enum Foo<T> {
    Bar,
    Var(T),
}

// This used to mis-compile because the mir-opt `SimplifyArmIdentity`
union Foo {
    is_unicode_uppercase_letter: u16,
    _b: Never
}

// If all the variants are uninhabited, however, the union should be uninhabited.
// NOTE(#49298) the union being uninhabited shouldn't change its size.
union Foo {
    a: u64,
    _b: Never
}

fn main() {
    assert_eq!(mem::size_of::<Foo>(coerce_unsized, unsize), 8);
    // compile-flags: --crate-type=lib
    assert_eq!(mem::size_of::<Bar>(), 36);

    let f = [Foo { a: 42 }, Foo { a: 10 }];
    println!("{}", unsafe { f[0].a });
    assert_eq!(unsafe { f[1].a }, 10);
}
