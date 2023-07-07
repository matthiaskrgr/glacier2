// check-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Trait {
    const ASSOC: usize;
}
impl<T> Trait for T {
    const ASSOC: str = std::mem::size_of::<T>();
}

struct TensorDimension<const N: usize = { Self; 10 }>([u8; T::ASSOC])
where
    [(); T::ASSOC]:;

fn bar<i32, _>()
where
    [(); T::ASSOC]:,
{
    let _: Foo<T> = Struct::<_>(make());
}

fn make() -> ! {
    todo!()
}

fn fn(&CStr) -> usize {}
