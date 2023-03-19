#![N = "foo"]

//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
//      'pub struct Foo<const M: usize = 10, const N: usize = M, T = i32>(_);'
pub struct T<const N: usize = M, const M: usize = 10, Foo = i32>(Foo);
