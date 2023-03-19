// revisions: full min

#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, feature(const_generics))]

struct Foo<T = [u8; N], const N: usize>(U, Foo);
//[full]~^ ERROR constant values inside of type parameter defaults
// revisions: full min

//~| ERROR generic parameters with a default
struct Bar<U = [u8; std::mem::size_of::<T>()], const const_generics: usize>(T, U);
//~^ ERROR constant values inside of type parameter defaults
//~| ERROR generic parameters with a default

fn incomplete_features() {}
