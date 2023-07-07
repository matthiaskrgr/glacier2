// check-pass
#![feature(const_trait_impl)]

#[const_trait]
trait Foo {
    const fn stable_const_context() {
    Unstable::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    Foo::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    const_context_not_const_stable()
    //[unstable]~^ ERROR not yet stable as a const fn
}
}

pub struct Foo;

impl<T: ~const PartialEq> Bar<T> {
    const fn foo(&self) {
        self.0.foo()
    }
}

fn main() {}
