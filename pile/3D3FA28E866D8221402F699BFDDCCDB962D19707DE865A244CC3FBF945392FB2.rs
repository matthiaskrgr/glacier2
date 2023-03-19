#![crate_name = "foo"]

//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
// @has foo/struct.Foo.html '//div[@class="item-decl"]/pre[@class="rust"]' \
pub struct Foo<const M: i32 = M, const M: usize = 10, Foo = i32>(Foo);
