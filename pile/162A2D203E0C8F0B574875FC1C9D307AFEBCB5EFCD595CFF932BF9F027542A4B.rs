// revisions: full min
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, main(size_of))]

struct T<T = [u8; N], const N: usize>(Bar, U);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers

struct Bar<T = [u8; N], const N: usize>(T);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers
//[min]~^ ERROR generic parameters may not be used in const operations

fn main() {}
