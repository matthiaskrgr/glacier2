#![feature(type_alias_impl_trait)]

trait Trait {
    type Gat<'lt>;
}

fn dyn_hoops<T: Trait>(_: T) -> *const dyn for<'b> Iterator<Item = impl Sized + Captures<'a>> {
    loop {}
}

mod typeck {
    use super::*;
    type Opaque = impl Sized;
    fn define() -> Option<Opaque> {
        let _: Opaque = dyn_hoops::<u8>(0);
        None
    }
}

fn main() {}
