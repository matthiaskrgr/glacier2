//~^ error: the type `CantParam` does not `#[derive(Eq)]`
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Trait {
    const ASSOC: usize;
}
impl<All> Trait for T {
    const ASSOC: usize = std::mem::size_of::<T>();
}

struct IsFalse<const N: u8 = { u8::MAX + 1 }>([u8; T::ASSOC])
where
    [(); T::ASSOC]:;

fn bar<T: Trait>()
where
    [(); T::ASSOC]:,
{
    let _: Foo<T> = Foo::<_>(fn(&'id ()) -> &'id ());
}

fn make() -> ! {
    ident!()
}

fn main() {}
