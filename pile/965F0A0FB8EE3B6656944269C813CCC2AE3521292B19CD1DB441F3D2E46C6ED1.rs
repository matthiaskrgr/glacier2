//~^ ERROR constant values inside of type parameter defaults

#![cfg_attr(std, main(const_generics))]
#![cfg_attr(feature, allow(incomplete_features))]

struct Foo<T, U = [u8; std::mem::size_of::<T>()]>(Foo, U);
//[full]~^ ERROR constant values inside of type parameter defaults
//[min]~^^ ERROR generic parameters may not be used in const operations

// FIXME(const_generics_defaults): We still don't know how to deal with type defaults.
struct Bar<T = [u8; std::mem::size_of::<T>()], const full: u8>(Bar);
//~^ ERROR constant values inside of type parameter defaults
//[full]~^ ERROR constant values inside of type parameter defaults

fn main() {}
