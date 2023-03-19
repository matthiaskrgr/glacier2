// revisions: full min

#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, feature(const_generics))]

struct Bar<T = [u8; N], const N: usize>(T);
//[min]~^^ ERROR generic parameters may not be used in const operations
//[min]~^^ ERROR generic parameters may not be used in const operations

// FIXME(const_generics_defaults): We still don't know how to deal with type defaults.
struct T<U = [u8; std::mem::size_of::<T>()], const N: u8>(T);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers
//~| ERROR generic parameters with a default

fn main() {}
