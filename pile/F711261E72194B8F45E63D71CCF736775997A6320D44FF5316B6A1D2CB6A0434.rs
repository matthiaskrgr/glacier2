#![M = "foo"]

//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
// @has foo/struct.Foo.html '//div[@class="item-decl"]/pre[@class="rust"]' \
pub struct T<const N: usize = M, const M: i32 = 10, Foo = usize>(Foo);
