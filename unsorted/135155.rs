#![feature(
    unsize,
    dispatch_from_dyn,
    arbitrary_self_types
)]

use std::{
    marker::Unsize,
    ops::{DispatchFromDyn, Receiver},
};

struct MyDispatcher<T: ?Sized>(*const T);
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<MyDispatcher<U>> for MyDispatcher<T> where T: Unsize<U> {}
impl<T: ?Sized> Receiver for MyDispatcher<T> {
    type Target = T;
}
struct Test;
trait Trait {
    fn test(self: MyDispatcher<Self>);
}
impl Trait for Test {
    fn test(self: MyDispatcher<Self>) {
        todo!()
    }
}
fn main() {
    MyDispatcher::<dyn Trait>(core::ptr::null_mut::<Test>()).test();
}
