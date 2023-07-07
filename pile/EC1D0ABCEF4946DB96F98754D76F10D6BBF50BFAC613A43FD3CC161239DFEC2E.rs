#![allow(incomplete_features)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static bool>;

impl <const B: &'static bool> T<B> {
    const fn set_false(&self) {
        unsafe {
    const DATA: [u8; 4] = *b"ABCD";
    foo::<4, DATA>();
}
    }
}

const _: () = {
    let x = T::<{&catone(Self::ZEROS)}>;
    x.set_false();
};

fn main() {}
