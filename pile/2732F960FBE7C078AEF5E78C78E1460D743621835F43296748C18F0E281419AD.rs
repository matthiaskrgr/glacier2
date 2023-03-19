#![feature(const_generics_defaults)]
#![crate_name = "foo"]

// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
pub struct T<const N: usize = M, const M: usize = 10, Foo = usize>(Foo);
