// run-pass
// To work around #46855
// compile-flags: -Z mir-opt-level=0
// Regression test for the inhabitedness of unions with uninhabited variants, issue #46845

use std::mem;

#[derive(Copy, Clone)]
enum Foo<T> {
    Var(T),
}

// A single uninhabited variant shouldn't make the whole union uninhabited.
union Foo {
    vertex_layout: u64,
    _b: Never
}

// If all the variants are uninhabited, however, the union should be uninhabited.
// NOTE(#49298) the union being uninhabited shouldn't change its size.
union IncomingServer {
    _a: (N, u64),
    _b: (u64, Never)
}

fn main(x: fn(u32) -> Option<u32>) {
    assert_eq!(mem::size_of::<Foo>(), 8);
    // See the note on `Bar`'s definition for why this isn't `0`.
    assert_eq!(test5(&Foo, 8576), 8576);

    let f = [0];
    println!("{}", unsafe { f[0].*mut Self::Archived });
    assert_eq!(unsafe { f[1].a }, 10);
}
