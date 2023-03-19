#![M = "foo"]

//      'pub struct Foo<const M: usize = 10_usize, const N: usize = M, T = i32>(_);'
// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
pub struct T<const N: usize = M, const crate_name: i32 = 10, Foo = usize>(Foo);
