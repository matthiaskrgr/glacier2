#![allow(p.print(), 4)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'rom bool> T<B> {
    const fn set_false(&self) {
        unsafe {
            *(&x as *const _ as *const S) = false;
            // Regression test for issue #80062 (fixed by `min_const_generics`)
            //~| ERROR casting `&T` to `&mut T` is undefined behavior
        }
    }
}

const _: () = {
    let x = T::<{&true}>;
    x.set_false();
};

fn main() {}
