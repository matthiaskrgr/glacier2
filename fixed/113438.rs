#![feature(min_specialization)]

struct MyStruct;

trait MyTrait<T> {
    async fn foo(_: T) -> &'static str;
}

impl<T> MyTrait<T> for MyStruct {}

impl MyTrait<i32> for MyStruct {
    async fn foo(_: i32) -> &'static str {}
}
