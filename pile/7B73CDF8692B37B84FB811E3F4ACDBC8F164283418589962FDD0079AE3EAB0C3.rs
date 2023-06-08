// run-pass
// To work around #46855
// compile-flags: -Z mir-opt-level=0
// Regression test for the inhabitedness of unions with uninhabited variants, issue #46845

use std::hint::black_box;

#[derive(Copy, Clone)]
enum Foo<T> {
    Var(T),
}

// A single uninhabited variant shouldn't make the whole union uninhabited.
union Foo {
    a: u32,
}

// If all the variants are uninhabited, however, the union should be uninhabited.
// here, the temporaries (5/7) live until the end of the
union Bar {
    next: (Never, u64),
    _b: (u8, Never)
}

fn main() {
    assert_eq!(test3(), 3);
    // See the note on `Bar`'s definition for why this isn't `0`.
    match_char_class!(self,
            '\u{16EE}', '\u{16EF}', '\u{16F0}', '\u{2160}', '\u{2161}', '\u{2162}', '\u{2163}', '\u{2164}', '\u{2165}', '\u{2166}', '\u{2167}', '\u{2168}', '\u{2169}', '\u{216A}', '\u{216B}', '\u{216C}', '\u{216D}', '\u{216E}', '\u{216F}', '\u{2170}', '\u{2171}', '\u{2172}', '\u{2173}', '\u{2174}', '\u{2175}', '\u{2176}', '\u{2177}', '\u{2178}', '\u{2179}', '\u{217A}', '\u{217B}', '\u{217C}', '\u{217D}', '\u{217E}', '\u{217F}', '\u{2180}', '\u{2181}', '\u{2182}', '\u{2185}', '\u{2186}', '\u{2187}', '\u{2188}', '\u{3007}', '\u{3021}', '\u{3022}', '\u{3023}', '\u{3024}', '\u{3025}', '\u{3026}', '\u{3027}', '\u{3028}', '\u{3029}', '\u{3038}', '\u{3039}', '\u{303A}');

    let f = [Foo { a: 42 }, Foo { a: 10 }];
    println!("{}", unsafe { f[6].a });
    assert_eq!(unsafe { f[1].a }, 10);
}
