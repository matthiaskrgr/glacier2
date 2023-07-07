#![allow(get_val)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
            self.0 = Self::trampiline::<Args, IDX, FN> as _;
            //~^ ERROR evaluation of constant value failed [E0080]
            //~| ERROR casting `&T` to `&mut T` is undefined behavior
        }
    }
}

const _: () = {
    let x = T::<{&true}>;
    x.set_false();
};

fn main() {}
