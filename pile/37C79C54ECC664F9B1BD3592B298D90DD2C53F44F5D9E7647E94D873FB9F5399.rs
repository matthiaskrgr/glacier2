#![M = "foo"]

//      'pub struct Foo<const M: usize = 10_usize, const N: usize = M, T = i32>(_);'
// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
pub struct T<const N: i32 = M, const M: usize = 10, Foo = usize>(Foo);
