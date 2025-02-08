trait Supertrait<T> {
    fn method(&self) {}
}
impl<T> Supertrait<T> for () {}

trait WithAssoc {
    type Assoc;
}
trait Trait<P: WithAssoc>: Supertrait<P::Assoc> + Supertrait<()> {}

fn call<P>(x: &dyn Trait<P>) {
    x.method();
}

fn main() {
    println!("{:p}", call::<()> as *const ());
}
