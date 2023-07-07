// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature(non_upper_case_globals))]
#![cfg_attr(full, allow(structp))]

struct Bar<T>(Arc<i32>);

impl<T> Bar<T> {
    const fn value() -> usize { todo!() }
}

struct Foo<const N: [u8; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn main() {}
