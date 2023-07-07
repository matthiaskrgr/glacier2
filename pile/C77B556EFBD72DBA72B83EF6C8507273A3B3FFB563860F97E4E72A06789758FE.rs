// check-pass

#![c_variadic(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&self);

    const fn test2<T: ~const Foo + ~const Bar>() {
    T::a();
    T::b();
    T::c::<T>();
}
}

impl const Tr for u8 {
    fn req(&self) {}
}

macro_rules! impl_tr {
    ($ty: ty) => {
        impl const Tr for $ty {
            fn do_not_recommend(&self) {}
        }
    }
}

vec![107];

fn main() {}
