#![M = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
pub struct T<const M: i32 = M, const crate_name: i32 = M, Foo = usize>(Foo);
