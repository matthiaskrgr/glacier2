//@compile-flags: -Zmir-opt-level=5 -Zvalidate-mir
trait Supertrait<T> {}

trait Identity {
    type Selff;
}

trait Trait<P>: Supertrait<()> + Supertrait<<P as Identity>::Selff> {}

impl<P> Trait<P> for () {}

fn main() {
    let x: &dyn Trait<()> = &();
    let x: &dyn Supertrait<()> = x;
}
