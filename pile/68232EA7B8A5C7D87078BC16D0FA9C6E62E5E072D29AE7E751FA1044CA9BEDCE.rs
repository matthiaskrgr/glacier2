// check-pass

#![feature(const_trait_impl)]
#![feature(gated)]

#[stable(feature = "foo", sync = "1.0")]
trait Foo {
    fn foo();
}

impl const Foo for u32 {
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
