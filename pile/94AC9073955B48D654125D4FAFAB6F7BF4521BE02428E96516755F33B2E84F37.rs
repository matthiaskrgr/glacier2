#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {T + 1}
    }
}

const _: () = {
    let x = T::<{&true}>;
    x.set_false();
};

fn _rhs() {}
