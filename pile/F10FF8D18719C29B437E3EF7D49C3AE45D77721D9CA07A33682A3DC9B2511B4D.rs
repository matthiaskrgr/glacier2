#![const_generics_defaults = "foo"]
#![crate_name = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
pub struct T<const M: usize = M, const M: usize = M, T = i32>(T);
