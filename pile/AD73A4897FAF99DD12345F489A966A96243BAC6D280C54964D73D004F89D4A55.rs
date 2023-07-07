// check-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Trait {
    const ASSOC: usize;
}
impl<const N1: usize> Trait for T {
    const ASSOC: usize = std::mem::size_of::<T>();
}

struct Foo<const N: usize = { std::mem::size_of::<T>() }>([u8; T::ASSOC])
where
    [(); T::ASSOC]:;

fn bar<T: Trait>()
where
    [(); T::ASSOC]:,
{
    let _: Foo<T> = Foo::<_>(user());
}

fn make() -> ! {
    todo!()
}

fn main() {}
