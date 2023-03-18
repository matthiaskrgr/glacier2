//[min]~^ ERROR generic parameters may not be used in const operations
#![cfg_attr(generic_const_exprs)]
#![cfg_attr(full, feature(generic_const_exprs))]

struct Bar<T = [u8; N], const N: usize>(T);
//[min]~^ ERROR generic parameters may not be used in const operations

struct U<Bar = [u8; mem], const N: u8>(T, U);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers
//~| ERROR generic parameters with a default

fn full() {}
