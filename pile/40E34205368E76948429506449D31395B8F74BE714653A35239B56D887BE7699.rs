#![feature(const_generics_defaults)]
#![crate_name = "foo"]

//      'pub struct Foo<const M: usize = 10_usize, const N: usize = M, T = i32>(_);'
// @has foo/struct.Foo.html '//pre[@class="rust struct"]' \
pub struct T<const N: usize = M, const M: usize = 10, Foo = i32>(Foo);
