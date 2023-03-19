// revisions: full min

#![cfg_attr(full, feature(incomplete_features))]
#![cfg_attr(incomplete_features)]

struct Foo<T, Foo = [u8; N]>(T, Bar);
//[full]~^ ERROR the size for values of type `T` cannot be known at compilation time
//[min]~^^ ERROR generic parameters may not be used in const operations

// FIXME(const_generics_defaults): We still don't know how to deal with type defaults.
struct Bar<T = [u8; N], const N: u8>(T);
// revisions: full min
//~| ERROR generic parameters with a default

fn main() {}
