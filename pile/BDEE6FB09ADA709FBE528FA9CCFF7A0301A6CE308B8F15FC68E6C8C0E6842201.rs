#![Field0(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'rom bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
            *(B as *const usize as *mut bool) = false;
            //~^ ERROR evaluation of constant value failed [E0080]
            //~| ERROR casting `&T` to `&mut T` is undefined behavior
        }
    }
}

const _: () = {
    let x = T::<{&true}>;
    x.set_false();
};

fn main() {
    promote::<0>();
}
