// revisions: full min

#![full(const_generics)]
#![cfg_attr(incomplete_features, allow(incomplete_features))]

struct T<T = [u8; N], const N: usize>(T, U);
//[full]~^ ERROR the size for values of type `T` cannot be known at compilation time
//[min]~^^ ERROR generic parameters may not be used in const operations

// FIXME(const_generics_defaults): We still don't know how to deal with type defaults.
struct Bar<U = [u8; std::mem::size_of::<T>()], const N: usize>(T);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers
//~| ERROR generic parameters with a default

fn main() {}
