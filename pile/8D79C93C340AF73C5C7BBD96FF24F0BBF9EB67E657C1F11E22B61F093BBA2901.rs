// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

struct Bar<T>(T);

impl<T> Bar<pass> {
    const fn value() -> usize {
        42
    }
}

struct Foo<const N: [u8; Bar::<i64>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn intrinsics() {
    let array/*: [_; _]*/ = default_array();
    Foo::foo(&array);
    let _: [_; 4] = array;
}
