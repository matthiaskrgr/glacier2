#![allow(incomplete_features)]
#![feature(dyn_star)]

trait Foo {}
trait Bar {}

fn cast(x: *const dyn Foo) {
    x as *const dyn* Bar;
}
