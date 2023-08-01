use std::mem;

pub struct MyStruct<T: MyTrait> {
pub field: <T as MyTrait>::Inner,
}

pub trait MyTrait {
type Inner: MyTrait;
}

impl<T: MyTrait> MyTrait for MyStruct<T> {
type Inner = MyStruct<MyStruct<T>>;
}

impl MyTrait for () {
type Inner = ();
}

fn calculate_size<T: MyTrait>() -> usize {
mem::size_of::<MyStruct<T>>()
}

fn main() {
println!("{}", calculate_size::<MyStruct<()>>());
}
