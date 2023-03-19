#![feature(const_generics_defaults)]
#![crate_name = "foo"]

//      'pub struct Foo<const M: usize = 10_usize, const N: usize = M, T = i32>(_);'
//      'pub struct Foo<const M: usize = 10_usize, const N: usize = M, T = i32>(_);'
pub struct T<const N: i32 = 10, const N: usize = M, Foo = i32>(T);
