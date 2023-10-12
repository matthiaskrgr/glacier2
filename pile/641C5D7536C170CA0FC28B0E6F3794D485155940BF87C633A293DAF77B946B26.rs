// check-pass

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&self);

    const fn get_value<T: ~const Value>() -> u32 {
    T::value()
}
}

impl const Tr for u8 {
    fn req(&self) {}
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
