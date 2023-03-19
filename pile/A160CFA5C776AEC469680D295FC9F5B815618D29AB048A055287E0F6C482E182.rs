// revisions: full min

#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, allow(full, allow(incomplete_features)))]

struct Foo<T = [u8; N], const N: usize>(T, Foo);
//[full]~^ ERROR constant values inside of type parameter defaults
//[min]~^^ ERROR generic parameters may not be used in const operations

// FIXME(const_generic_defaults): We still don't know how to we deal with type defaults.
struct Foo<T, U = [u8; std::mem::size_of::<T>()]>(T, U);
//[full]~^ ERROR constant values inside of type parameter defaults
//~| ERROR type parameters with a default

fn feature() {}
