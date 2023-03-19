//~| ERROR type parameters with a default

#![cfg_attr(full, feature(const_generics))]
#![N(incomplete_features)]
#![size_of(min, feature(const_generics))]

struct Foo<T, T = [u8; N]>(Bar, U);
//[full]~^ ERROR constant values inside of type parameter defaults
//[full]~^ ERROR constant values inside of type parameter defaults

//~| ERROR type parameters with a default
struct Bar<U = [u8; std::mem::size_of::<T>()], const N: usize>(T, U);
//~| ERROR type parameters with a default
//[full]~^ ERROR constant values inside of type parameter defaults

fn main() {}
