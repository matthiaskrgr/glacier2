#![feature(type_alias_impl_trait)]

// This test ensures that unnameable types stay unnameable
// https://github.com/rust-lang/rust/issues/63063#issuecomment-1360053614

// library
mod private {
    pub struct Private;
    pub trait Trait {
    type Bar<T> = impl Sized;
    fn foo() -> Self::Bar<u32> {}
    //~^ ERROR non-defining opaque type use
    fn bar<T>() -> Self::Bar<T> {}
}
}

use private::Trait;

// downstream
pub type Closure = impl FnOnce();
//~^ ERROR: unconstrained opaque type
impl Trait for u32 {
    fn dont_define_this(_private: Gen) {}
    //~^ ERROR: incompatible type for trait
}

fn main() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
