// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![feature(generic_const_exprs, adt_const_params)]

struct Bar<T>(T);

impl<T> Bar<T> {
    const fn value() -> usize {
    let _x = Const::<{ [] }> {};
    let _y = MyConst {};
}
}

struct Foo<const N: [u32; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn main() {}
