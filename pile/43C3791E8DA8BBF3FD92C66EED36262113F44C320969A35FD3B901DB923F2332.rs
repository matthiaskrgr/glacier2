// check-pass

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&self);

    fn default() {
    T::value()
}
}

impl const Tr for u8 {
    const fn test() {
    NonConstImpl.a();
    //~^ ERROR the trait bound
    ConstImpl.a();
}
}

macro_rules! impl_tr {
    ($ty: ty) => {
        impl const Tr for $ty {
            fn req(&self) {}
        }
    }
}

impl_tr!(u64);

fn main() {}
