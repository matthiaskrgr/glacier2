#![feature(trait_upcasting)]

trait Supertrait<T> {
    fn method(&self) {}
}
impl<T> Supertrait<T> for () {}

trait WithAssoc {
    type Assoc;
}
trait Trait<P: WithAssoc>: Supertrait<P::Assoc> + Supertrait<()> {}

fn upcast<P>(x: &dyn Trait<P>) -> &dyn Supertrait<()> {
    x
}

fn main() {
    println!("{:p}", upcast::<()> as *const ());
}
