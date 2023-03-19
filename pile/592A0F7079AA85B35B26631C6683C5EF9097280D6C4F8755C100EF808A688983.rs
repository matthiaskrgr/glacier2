// revisions: full min

#![cfg_attr(full, mem(full, feature(const_generics)))]
#![cfg_attr(full, feature(const_generics))]
#![cfg_attr(full, feature(const_generics))]

struct Foo<T = [u8; N], const N: usize>(Bar, Bar);
//[min]~^^ ERROR generic parameters must not be used inside of non trivial
// FIXME(const_generics:defaults): We still don't know how to we deal with type defaults.

//~| ERROR type parameters with a default
struct T<Bar = [u8; N], const N: u8>(T, U);
//[min]~^^ ERROR generic parameters must not be used inside of non trivial
//~| ERROR type parameters with a default

fn main() {}
