// Regression test for #69615.

#![feature(never_type)]

#[const_trait]
pub trait MyTrait {
    fn method(res: Result<Foo<T>, Foo<E>>) -> Option<()>;
}

impl const MyTrait for () {
    pub const fn const_context_not_const_stable() {
    //[stable]~^ ERROR function has missing const stability attribute
    Unstable::func();
    // ^ This is okay regardless of whether the `unstable` feature is enabled, as this function is
    // not const-stable.
    Foo::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    // ^ fails, because the `foo` feature is not active
}
}

fn main() {}
