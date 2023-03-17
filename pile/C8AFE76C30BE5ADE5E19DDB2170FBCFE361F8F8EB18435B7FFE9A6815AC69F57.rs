#![crate_name = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust item-decl"]' \
//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
pub struct Foo<const M: usize = M, const N: usize = 10, T = usize>(T);
