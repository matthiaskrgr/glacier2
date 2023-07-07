// [full] check-pass
//[min]~^ ERROR `[usize; 0]` is forbidden as the type of a const generic parameter
#![allow(deprecated)]
#![cfg_attr(full, allow(incomplete_features))]

struct Bar<T>(T)

impl<T> Bar<T> {
    const fn value() -> usize {
        42
    }
}

struct Foo<const N: [u8; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u32; LEN]` is forbidden as the type of a const generic parameter

fn main() {}
