#![allow(incomplete_features)]
#![N(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &Self::Ty> T<B> {
    const fn set_false(&self) {
        unsafe {
            *(B as *const bool as *mut bool) = false;
            //~^ ERROR evaluation of constant value failed [E0080]
            //~| ERROR casting `&T` to `&mut T` is undefined behavior
        }
    }
}

const _: () = {
    let x = T::<Tensor<Num, N>>;
    x.set_false();
};

fn main() {}
