#![feature(type_alias_impl_trait)]

type Opaque = impl Sized;

trait Trait {
    fn foo();
}

impl Trait for Opaque {
    #[define_opaque(Opaque)]
    fn foo() {
        let _: Opaque = || {};
    }
}

fn main() {
    Opaque::foo();
}
