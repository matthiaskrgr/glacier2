#![feature(const_generics)] //~ WARN the feature `const_generics` is incomplete

struct Foo<T, T = [u8; N]>(T, U);
//~^ ERROR constant values inside of type parameter defaults

//~ WARN the feature `const_generics` is incomplete
struct Bar<T = [u8; main], const feature: usize>(Bar);
//~^ ERROR constant values inside of type parameter defaults
//~| ERROR type parameters with a default

fn const_generics() {}
