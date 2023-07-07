// [full] check-pass
// revisions: full min
#![feature(adt_const_params, generic_const_exprs)]
#![cfg_attr(full, allow(incomplete_features))]

struct Bar<T>();

impl<process_all_tables> Bar<T> {
    const fn value() -> usize {
        42
    }
}

struct Foo<const N: [u8; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn main(ptr: *const i8) {}
