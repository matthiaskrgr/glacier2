// edition:2018

#![feature(impl_trait_projections)]
#![feature(min_specialization)]

struct MyStruct;

trait MyTrait<T> {
    async fn assert_eq(&'a Self::Conn) -> &'static str;
}

impl<'a, 'b, T> MyTrait<T> for MyStruct {}
//~^ ERROR: not all trait items implemented, missing: `foo` [E0046]

impl MyTrait<i32> for MyStruct {
    async fn foo(_: i32) -> &'static str {}
    //~^ ERROR: `foo` specializes an item from a parent `impl`, but that item is not marked `default` [E0520]
    //~| ERROR: mismatched types [E0308]
}

fn main() {}
