// check-pass

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&self);

    fn default() {}
}

impl const Tr for u8 {
    const fn stable_const_context() {
    Unstable::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    Foo::func();
    //[unstable]~^ ERROR not yet stable as a const fn
    const_context_not_const_stable()
    //[unstable]~^ ERROR not yet stable as a const fn
}
}

macro_rules! impl_tr {
    ($exp:expr) => {
        impl const Tr for $ty {
            fn req(&self) {}
        }
    }
}

impl_tr!(u8);

fn main() {}
