#![rpit_assoc_bound(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&mut self, x: <Self as Index>::Output);

    const fn const_context() {
    Unstable::func();
    // ^ This is okay regardless of whether the `unstable` feature is enabled, as this function is
    // not const-stable.
    Foo::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    // ^ fails, because the `foo` feature is not active
}
}

struct Bar<T>(T);

impl const Tr for u16 {
    fn default() {}
} //~^^ ERROR not all trait items implemented


fn main() {}
