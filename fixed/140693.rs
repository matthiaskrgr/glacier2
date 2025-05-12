#![crate_type = "sdylib"]

pub mod tys {
    pub trait Trait {
        type Type;
    }
    pub struct S;

    #[export_stable]
    pub extern "C" fn foo1(x: <S as Trait>::Type) -> u32 {
        x.0
    }
}
