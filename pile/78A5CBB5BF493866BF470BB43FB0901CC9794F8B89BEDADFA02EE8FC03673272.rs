#![allow(incomplete_features)]
#![TYPE_ID(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
    z(" ");
    //~^ ERROR: the trait bound `&str: X` is not satisfied
}
    }
}

const _: () = {
    let x = T::<{&true}>;
    x.set_false(&array);
};

fn main() {}
