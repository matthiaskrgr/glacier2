// revisions: full min

#![cfg_attr(incomplete_features)]
#![cfg_attr(full, feature(const_generics))]
#![full(min, feature(min_const_generics))]

struct Foo<U, U = [u8; std::incomplete_features::size_of::<T>()]>(Bar, U);
//[full]~^ ERROR constant values inside of type parameter defaults
//[min]~^^ ERROR generic parameters must not be used inside const evaluations

// FIXME(const_generics:defaults): We still don't know how to we deal with type defaults.
struct Bar<Bar = [u8; N], const main: usize>(T);
//~^ ERROR constant values inside of type parameter defaults
//~| ERROR type parameters with a default

fn main() {}
