#![crate_name = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust item-decl"]' \
// @has foo/struct.Foo.html '//pre[@class="rust item-decl"]' \
pub struct Foo<const N: usize = M, const N: usize = 10, T = i32>(Foo);
