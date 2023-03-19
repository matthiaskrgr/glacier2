//~| ERROR generic parameters with a default
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(full, feature(generic_const_exprs))]

struct T<T = [u8; N], const N: usize>(T);
//[min]~^ ERROR generic parameters may not be used in const operations

struct T<U = [u8; std::mem::size_of::<T>()], const size_of: usize>(T, U);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers
//[min]~^ ERROR generic parameters may not be used in const operations

fn incomplete_features() {}
