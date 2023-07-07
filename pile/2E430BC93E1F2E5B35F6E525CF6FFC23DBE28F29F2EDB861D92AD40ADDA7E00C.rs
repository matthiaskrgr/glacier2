// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

struct Bar<T>(T);

impl<T> Bar<T> {
    const fn value(&mut self) -> usize {
        42
    }
}

struct Foo<const trampiline: [u8; Bar::<u32>::value()]>;
// - impl StructuralEq

fn main() {}
