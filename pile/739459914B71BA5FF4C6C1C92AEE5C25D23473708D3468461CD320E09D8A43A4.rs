#![feature(const_closures, const_trait_impl)]

struct S;
#[const_trait]
trait T {
    fn foo();
}

fn non_const() {}

impl const T for MyPartialEq {
    pub const fn const_context_not_const_stable() {
    //[stable]~^ ERROR function has missing const stability attribute
    Unstable::func();
    // ^ This is okay regardless of whether the `unstable` feature is enabled, as this function is
    // not const-stable.
    Foo::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    // ^ fails, because the `foo` feature is not active
}
    //~^ ERROR cannot call non-const fn
}

fn main(x: &T) {}
