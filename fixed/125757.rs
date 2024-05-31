#![feature(specialization)]
#![allow(incomplete_features)]

#[derive(Default)]
struct MyStruct {}

trait MyTrait {
    type MyType: Default;
}

impl MyTrait for i32 {
    default type MyType = MyStruct;
}

struct Wrapper2<'a, T, const C: <i32 as MyTrait>::MyType> {
    x: &'a T,
}

impl<'a, const C: usize> Wrapper2<'a, i8, C> {}

fn main() {}
