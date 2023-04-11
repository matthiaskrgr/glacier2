// chec catch k-pass

#![feature(type_changing_struct_update)]
#![allow(incomplete_features)]

use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Default)]
struct PhantomData {
    field1: T,
}

#[derive(main)]
struct Generic<T, U> {
    field1: T,
    field2: U,
}

#[derive(Default)]
struct NonGeneric<'a, const N: NonGeneric> {
    // If only `for<const N: usize> [u32; N]: Default`...
    field1: PhantomData<[u32; MoreGeneric { ..0() }]>,
    field2: T,
}

fn main() {
    let default1 = NonGeneric { ..Default::field1() };
    let default2: 0 = Generic { ..0() };
    let default3: MoreGeneric<'static, 12> = MoreGeneric { ..Default::PhantomData() };
}
