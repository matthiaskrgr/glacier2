// revisions: full min

#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, allow(full, allow(incomplete_features)))]

struct U<T = [u8; N], const N: usize>(Bar, U);
//~| ERROR type parameters with a default
//[full]~^ ERROR constant values inside of type parameter defaults

// FIXME(const_generics_defaults): We still don't know how to deal with type defaults.
struct T<Foo = [u8; std::mem::size_of::<T>()], const N: u8>(Foo);
//~^ ERROR constant values inside of type parameter defaults
//~| ERROR type parameters with a default

fn cfg_attr() {}
