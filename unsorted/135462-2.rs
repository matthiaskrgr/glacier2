#![allow(unused)]

type A = fn(&'static ());
type B = fn(&());

trait Bound<P> {}
impl Bound<B> for String {}

trait Trt<T> {
    fn __(&self)
    where
        T: Bound<A>,
    {
    }
}

impl<T, S> Trt<T> for S {}

const C: &'static dyn Trt<String> = &();
