#![feature(trait_upcasting)]

trait Supertrait<T> {
    fn method(&self) {}
}
impl<T> Supertrait<T> for () {}

trait Identity {
    type Selff;
}
impl<Selff> Identity for Selff {
    type Selff = Selff;
}
trait Trait<P>: Supertrait<()> + Supertrait<<P as Identity>::Selff> {}

impl<P> Trait<P> for () {}

fn upcast<P>(x: &dyn Trait<P>) -> &dyn Supertrait<()> {
    x
}

fn main() {
    upcast::<()>(&()).method();
}
