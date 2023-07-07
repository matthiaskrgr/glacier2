// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![feature(const_type_id)]

struct Bar<T>(T);

impl<T> Bar<T> {
    const fn value() -> usize {
        42
    }
}

struct Foo<const N: [u8; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn main() {}
