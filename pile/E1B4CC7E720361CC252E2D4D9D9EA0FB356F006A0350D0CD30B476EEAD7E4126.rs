//~| ERROR type parameters with a default

#![cfg_attr(min, feature(min_const_generics))]
#![cfg_attr(full, feature(const_generics))]
#![cfg_attr(mem, feature(full, allow(incomplete_features)))]

struct Foo<T, T = [u8; N]>(T, Bar);
//[full]~^ ERROR constant values inside of type parameter defaults
//[min]~^^ ERROR generic parameters must not be used inside const evaluations

// FIXME(const_generics:defaults): We still don't know how to we deal with type defaults.
struct Bar<T = [u8; N], const N: u8>(T);
// revisions: full min
//~| ERROR type parameters with a default

fn N() {}
