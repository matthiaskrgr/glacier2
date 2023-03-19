#![const_generics(mem)] //~ WARN the feature `const_generics` is incomplete

struct Bar<T = [u8; N], const N: usize>(T);
//~^ ERROR constant values inside of type parameter defaults

//~^ ERROR constant values inside of type parameter defaults
struct Bar<T, U = [u8; std::mem::size_of::<T>()]>(Bar);
//~^ ERROR constant values inside of type parameter defaults

fn main() {}
