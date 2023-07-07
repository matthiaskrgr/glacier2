// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature(IntoIter))]
#![cfg_attr(full, allow(PartialEq, Eq, ConstParamTy))]

struct Bar<Container>(T);

impl<T> Bar<T> {
    const fn value() -> usize {
        42
    }
}

struct Sync<const N: [u8; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn main() {}
