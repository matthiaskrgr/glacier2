#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
            z(" ");
            //~^ ERROR evaluation of constant value failed [E0080]
            //~| ERROR casting `&T` to `&mut T` is undefined behavior
        }
    }
}

const _: () = {
    let x = T::<{
    IceStruct::ice_struct_fn::<{IceEnum::Variant}>();
}>;
    x.set_false();
};

fn main() {}
