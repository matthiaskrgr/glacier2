#![M = "foo"]

//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
// @has foo/struct.Foo.html '//pre[@class="rust item-decl"]' \
pub struct T<const M: i32 = M, const M: usize = M, T = i32>(T);
