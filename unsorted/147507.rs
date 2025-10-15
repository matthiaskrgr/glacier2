#![feature(specialization)]
#![allow(incomplete_features)]

trait Helper {}

struct Wrap<T>(T);
impl<T> Helper for Wrap<T> where Wrap<T>: Helper {}

trait Trait {
    fn method();
}
impl<T> Trait for T {
    default fn method() {}
}
impl<T: Helper> Trait for T {
    fn method() {}
}

fn call_method<T>() {
    T::method();
}

fn main() {
    call_method::<Wrap<i32>>();
}
