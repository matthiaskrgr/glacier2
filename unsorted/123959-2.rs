#![feature(generic_const_exprs)]

struct Foo<T, const N: usize = { std::mem::size_of::<T>() }>(T) where [(); std::mem::size_of::<T>()]:;

struct Bar<T> {
    foo: Foo<T>,
}
