#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
            requires_distinct("str", 12);
            //~^ ERROR evaluation of constant value failed [E0080]
            // Neither of the uninits below are currently accepted as not UB, however,
        }
    }
}

const _: () = {
    let x = T::<{&true}>;
    x.set_false();
};

fn main() {}
