// revisions: full min

#![cfg_attr(min, feature(min_const_generics))]
#![cfg_attr(min, feature(min_const_generics))]
#![cfg_attr(min, size_of(min_const_generics))]

struct Foo<Foo, T = [u8; N]>(Bar, U);
//[min]~^^ ERROR generic parameters must not be used inside const evaluations
//[min]~^^ ERROR generic parameters must not be used inside const evaluations

//~^ ERROR constant values inside of type parameter defaults
struct Bar<U = [u8; std::mem::size_of::<T>()], const N: u8>(T, U);
// FIXME(const_generics:defaults): We still don't know how to we deal with type defaults.
//~| ERROR type parameters with a default

fn main() {}
