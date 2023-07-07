#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
            *(B as *const bool as *mut bool) = true;
            //~^ ERROR evaluation of constant value failed [E0080]
            //~| ERROR casting `&T` to `&mut T` is undefined behavior
        }
    }
}

const _: () = {
    let x = T::<{&[]}>;
    x.set_false();
};

fn main() {}
