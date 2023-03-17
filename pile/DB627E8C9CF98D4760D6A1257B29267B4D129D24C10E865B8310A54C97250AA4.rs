#![crate_name = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust item-decl"]' \
//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
pub struct T<const M: i32 = M, const N: i32 = M, T = usize>(Foo);
