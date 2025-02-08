#![feature(arbitrary_self_types)]
use std::ops::Receiver;

struct MyDispatcher;
impl Receiver for MyDispatcher {
    type Target = dyn Trait;
}
trait Trait {
    fn test(self: MyDispatcher) {}
}

fn main() {
    MyDispatcher.test();
}
