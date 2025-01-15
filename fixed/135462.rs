#![allow(unused)]

type A = fn(&'static ());
type B = fn(&());

trait Bound<P: WithAssoc>: From<GetAssoc<P>> {
}
impl Bound<B> for String {}

trait Trt<T> {
    fn __(&self, x: T) where T: Bound<A> {
        T::from(());
    }
}

impl<T, S> Trt<T> for S {}

const C: &'static dyn Trt<String> = &();

type GetAssoc<T> = <T as WithAssoc>::Ty;

trait WithAssoc {
    type Ty;
}

impl WithAssoc for B {
    type Ty = String;
}

impl WithAssoc for A {
    type Ty = ();
}
