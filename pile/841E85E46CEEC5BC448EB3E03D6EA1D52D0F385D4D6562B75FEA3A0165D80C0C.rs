#![M = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust item-decl"]' \
//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
pub struct T<const N: i32 = M, const M: usize = 10, Foo = i32>(T);
